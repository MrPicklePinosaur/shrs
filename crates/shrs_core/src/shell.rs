//! Types for internal context of shell

use std::{
    cell::RefCell,
    env,
    path::{Path, PathBuf},
    process::ExitStatus,
    time::Instant,
};

use anyhow::anyhow;
use dirs::home_dir;
use log::{error, info, warn};
use shrs_job::JobManager;

use crate::{history::History, prelude::*};
/// Constant shell data
///
/// Data here is generally not mutated at runtime.
pub struct Shell {
    pub job_manager: RefCell<JobManager>,
    pub hooks: Hooks,
    /// Builtin shell functions that have access to the shell's context
    pub builtins: Builtins,
    /// Color theme
    pub theme: Theme,
    /// The command language
    pub lang: Box<dyn Lang>,
    /// Signals to be handled
    pub signals: Signals,
    pub keybinding: Box<dyn Keybinding>,
    pub plugin_metas: Vec<PluginMeta>,
}

/// Shared global shell context
///
/// Context here is shared by each subshell
// TODO can technically unify shell and context
pub struct Context {
    /// Output stream
    pub out: OutputWriter,
    pub state: State,
    pub jobs: Jobs,
    pub startup_time: Instant,
    pub alias: Alias,
    pub history: Box<dyn History>,
    pub prompt_content_queue: PromptContentQueue,

    pub completer: Box<dyn Completer>,
}

/// Runtime context for the shell
///
/// Contains data that can should be local to each subshell. Data here should also be able to be
/// cloned.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Runtime {
    /// Current working directory
    pub working_dir: PathBuf,
    /// Environment variables
    pub env: Env,
    /// Name of the shell or shell script
    pub name: String,
    /// Arguments this shell was called with
    pub args: Vec<String>,
    /// Exit status of most recent pipeline
    pub exit_status: i32,
    /// Directory for configuration files
    pub config_dir: PathBuf,
    // /// List of defined functions
    // pub functions: HashMap<String, Box<ast::Command>>,
}

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
    #[builder(default = "Alias::new()")]
    pub alias: Alias,

    /// Environment variables, see [Env]
    #[builder(default = "Env::default()")]
    pub env: Env,

    /// Completion system, see [Completer]
    #[builder(default = "Box::new(DefaultCompleter::new())")]
    #[builder(setter(custom))]
    completer: Box<dyn Completer>,

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
    pub history: Box<dyn History>,

    /// Keybindings, see [Keybinding]
    #[builder(default = "Box::new(DefaultKeybinding::default())")]
    #[builder(setter(custom))]
    pub keybinding: Box<dyn Keybinding>,

    /// Configuration directory, easy access in the shell
    #[builder(default = "home_dir().unwrap().join(\".config/shrs\")")]
    pub config_dir: PathBuf,
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
    pub fn with_history(mut self, history: impl History + 'static) -> Self {
        self.history = Some(Box::new(history));
        self
    }
    pub fn with_keybinding(mut self, keybinding: impl Keybinding + 'static) -> Self {
        self.keybinding = Some(Box::new(keybinding));
        self
    }
    pub fn with_completer(mut self, completer: impl Completer + 'static) -> Self {
        self.completer = Some(Box::new(completer));
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
            out: OutputWriter::new(self.theme.out_style, self.theme.err_style),

            state: self.state,
            jobs: Jobs::default(),
            startup_time: Instant::now(),
            history: self.history,
            prompt_content_queue: PromptContentQueue::new(),
            completer: self.completer,
        };
        let mut rt = Runtime {
            env: self.env,
            working_dir: std::env::current_dir().unwrap(),
            // TODO currently hardcoded
            name: "shrs".into(),
            // TODO currently unused (since we have not implemented functions etc)
            args: vec![],
            exit_status: 0,
            config_dir: self.config_dir,
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
            plugin_metas: plugins.iter().map(|p| p.meta()).collect(),
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

        run_shell(&sh, &mut ctx, &mut rt, &mut self.readline)
    }
}

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

/// Set the current working directory
pub fn set_working_dir(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    wd: &Path,
    run_hook: bool,
) -> anyhow::Result<()> {
    // Check working directory validity
    let path = if let Ok(path) = PathBuf::from(wd).canonicalize() {
        if !path.is_dir() {
            return Err(anyhow!("Invalid path"));
        }
        path
    } else {
        return Err(anyhow!("Invalid path"));
    };

    // Save old working directory
    let old_path = get_working_dir(rt).to_path_buf();
    let old_path_str = old_path.to_str().expect("failed converting to str");
    rt.env
        .set("OLDPWD", old_path_str)
        .expect("failed setting env var");

    let pwd = path.to_str().expect("failed converting to str");
    rt.env.set("PWD", pwd).expect("failed setting env var");
    rt.working_dir = path.clone();

    // Set process working directory too
    env::set_current_dir(path.clone()).expect("failed setting process current dir");

    // Run change directory hook
    if run_hook {
        let hook_ctx = ChangeDirCtx {
            old_dir: old_path,
            new_dir: path,
        };
        if let Err(e) = sh.hooks.run(sh, ctx, rt, hook_ctx) {
            error!("Error running change dir hook {e:?}");
        }
    }

    Ok(())
}

pub fn get_working_dir(rt: &Runtime) -> &Path {
    &rt.working_dir
}
