//! Shell configuration options

use std::{cell::RefCell, default, process::ExitStatus, time::Instant};

use ::crossterm::style::Color;
use log::{info, warn};
use shrs_core::prelude::*;
use shrs_job::JobManager;
use shrs_lang::PosixLang;
use shrs_line::prelude::*;

use crate::{
    history::{DefaultHistory, History},
    prelude::*,
};

/// Unified shell config struct
#[derive(Builder)]
#[builder(name = "ShellBuilder", pattern = "owned")]
#[builder(setter(prefix = "with"))]
pub struct ShellConfig {
    /// Runtime hooks, see [Hooks]
    #[builder(default = "Hooks::default()")]
    pub hooks: Hooks,

    /// Builtin shell commands, see [Builtins]
    #[builder(default = "Builtins::default()")]
    pub builtins: Builtins,

    /// Readline implementation
    #[builder(default = "Box::new(Line::default())")]
    #[builder(setter(custom))]
    pub readline: Box<dyn Readline>,

    /// Aliases, see [Alias]
    #[builder(default = "Alias::default()")]
    pub alias: Alias,

    /// Environment variables, see [Env]
    #[builder(default = "Env::default()")]
    pub env: Env,

    // /// List of defined functions
    // #[builder(default = "HashMap::new()")]
    // pub functions: HashMap<String, Box<ast::Command>>,
    /// Color theme
    #[builder(default = "Theme::default()")]
    pub theme: Theme,

    /// Command language
    #[builder(default = "Box::new(PosixLang::default())")]
    #[builder(setter(custom))]
    pub lang: Box<dyn Lang>,

    /// Plugins, see [Plugins]
    #[builder(default = "Vec::new()")]
    #[builder(setter(custom))]
    pub plugins: Vec<Box<dyn Plugin>>, // TODO could also maybe use anymap to get the concrete type

    /// Globally accessible state, see [State]
    #[builder(default = "State::default()")]
    #[builder(setter(custom))]
    pub state: State,

    /// History, see [History]
    #[builder(default = "Box::new(DefaultHistory::default())")]
    #[builder(setter(custom))]
    pub history: Box<dyn History<HistoryItem = String>>,

    /// Keybindings, see [Keybinding]
    #[builder(default = "Box::new(DefaultKeybinding::default())")]
    #[builder(setter(custom))]
    pub keybinding: Box<dyn Keybinding>,
}

impl ShellBuilder {
    pub fn with_plugin<P: std::any::Any + Plugin>(mut self, plugin: P) -> Self {
        let mut cur_plugins = self.plugins.unwrap_or_default();
        cur_plugins.push(Box::new(plugin));
        self.plugins = Some(cur_plugins);

        self
    }
    pub fn with_state<T: 'static>(mut self, state: T) -> Self {
        let mut cur_state = self.state.unwrap_or_default();
        cur_state.insert(state);
        self.state = Some(cur_state);
        self
    }
    pub fn with_lang(mut self, lang: impl Lang + 'static) -> Self {
        self.lang = Some(Box::new(lang));
        self
    }
    pub fn with_readline(mut self, line: impl Readline + 'static) -> Self {
        self.readline = Some(Box::new(line));
        self
    }
    pub fn with_history(mut self, history: impl History<HistoryItem = String> + 'static) -> Self {
        self.history = Some(Box::new(history));
        self
    }
    pub fn with_keybinding(mut self, keybinding: impl Keybinding + 'static) -> Self {
        self.keybinding = Some(Box::new(keybinding));
        self
    }
}

impl ShellConfig {
    /// Start up the shell
    ///
    /// This function contains the main loop of the shell and thus will block for the entire
    /// execution of the shell.
    pub fn run(mut self) -> anyhow::Result<()> {
        // TODO some default values for Context and Runtime are duplicated by the #[builder(default = "...")]
        // calls in ShellBuilder, so we are sort of defining the full default here. Maybe end
        // up implementing Default for Context and Runtime

        // run plugins first
        // TODO ownership issue here since other plugins can technically add plugins during init
        // process
        let plugins = self.plugins.drain(..).collect::<Vec<_>>();
        for plugin in plugins.iter() {
            let plugin_meta = plugin.meta();
            info!("Initializing plugin '{}'...", plugin_meta.name);

            if let Err(e) = plugin.init(&mut self) {
                // Error handling for plugin
                match plugin.fail_mode() {
                    FailMode::Warn => warn!(
                        "Plugin '{}' failed to initialize with {}",
                        plugin_meta.name, e
                    ),
                    FailMode::Abort => panic!(
                        "Plugin '{}' failed to initialize with {}",
                        plugin_meta.name, e
                    ),
                }
            }
        }

        let mut ctx = Context {
            alias: self.alias,
            out: OutputWriter::new(self.theme.out_color, self.theme.err_color),

            state: self.state,
            jobs: Jobs::default(),
            startup_time: Instant::now(),
            history: self.history,
        };
        let mut rt = Runtime {
            env: self.env,
            working_dir: std::env::current_dir().unwrap(),
            // TODO currently hardcoded
            name: "shrs".into(),
            // TODO currently unused (since we have not implemented functions etc)
            args: vec![],
            exit_status: 0,
            // functions: self.functions,
        };
        let sh = Shell {
            job_manager: RefCell::new(JobManager::default()),
            builtins: self.builtins,
            theme: self.theme,
            lang: self.lang,
            hooks: self.hooks,
            signals: Signals::new().unwrap(),
            keybinding: self.keybinding,
        };

        // run post init for plugins
        for plugin in plugins.iter() {
            if let Err(e) = plugin.post_init(&sh, &mut ctx, &mut rt) {
                let plugin_meta = plugin.meta();
                info!("Post-initializing plugin '{}'...", plugin_meta.name);

                // Error handling for plugin
                match plugin.fail_mode() {
                    FailMode::Warn => warn!(
                        "Plugin '{}' failed to post-initialize with {}",
                        plugin_meta.name, e
                    ),
                    FailMode::Abort => panic!(
                        "Plugin '{}' failed to post-initialize with {}",
                        plugin_meta.name, e
                    ),
                }
            }
        }

        let mut readline = self.readline;
        run_shell(&sh, &mut ctx, &mut rt, &mut readline)
    }
}

///
fn run_shell(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    readline: &mut Box<dyn Readline>,
) -> anyhow::Result<()> {
    // init stuff
    let res = sh.hooks.run::<StartupCtx>(
        sh,
        ctx,
        rt,
        StartupCtx {
            startup_time: ctx.startup_time.elapsed(),
        },
    );

    if let Err(_e) = res {
        // TODO log that startup hook failed
    }

    loop {
        let line = readline.read_line(sh, ctx, rt);

        // attempt to expand alias
        // TODO IFS
        let mut words = line
            .split(' ')
            .map(|s| s.trim_start_matches("\\\n").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if let Some(first) = words.get_mut(0) {
            let alias_ctx = AliasRuleCtx {
                alias_name: first,
                sh,
                ctx,
                rt,
            };

            // Currently only use the last alias, can also render a menu
            if let Some(expanded) = ctx.alias.get(&alias_ctx).last() {
                *first = expanded.to_string();
            }
        }
        let line = words.join(" ");

        // TODO not sure if hook should run here (since not all vars are expanded yet)
        let hook_ctx = BeforeCommandCtx {
            raw_command: line.clone(),
            command: line.clone(),
            run_ctx: rt.clone(),
        };
        sh.hooks.run::<BeforeCommandCtx>(sh, ctx, rt, hook_ctx)?;

        // Retrieve command name or return immediately (empty command)
        let cmd_name = match words.first() {
            Some(cmd_name) => cmd_name,
            None => continue,
        };

        let builtin_cmd = sh
            .builtins
            .iter()
            .find(|(builtin_name, _)| *builtin_name == cmd_name)
            .map(|(_, builtin_cmd)| builtin_cmd);

        let mut cmd_output: CmdOutput = CmdOutput::error();
        ctx.out.begin_collecting();
        if let Some(builtin_cmd) = builtin_cmd {
            let output = builtin_cmd.run(sh, ctx, rt, &words);
            match output {
                Ok(o) => cmd_output = o,
                Err(e) => eprintln!("error: {e:?}"),
            }
        } else {
            let output = sh.lang.eval(sh, ctx, rt, line.clone());
            match output {
                Ok(o) => cmd_output = o,
                Err(e) => eprintln!("error: {e:?}"),
            }
        }
        let (out, err) = ctx.out.end_collecting();
        cmd_output.set_output(out, err);
        let _ = sh.hooks.run(
            sh,
            ctx,
            rt,
            AfterCommandCtx {
                command: line,
                cmd_output,
            },
        );

        // check up on running jobs
        let mut exit_statuses = vec![];
        ctx.jobs.retain(|status: ExitStatus| {
            exit_statuses.push(status);
        });

        for status in exit_statuses.into_iter() {
            sh.hooks
                .run::<JobExitCtx>(sh, ctx, rt, JobExitCtx { status })?;
        }
    }
}
