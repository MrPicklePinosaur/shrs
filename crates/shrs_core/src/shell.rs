//! Core shell structs and shell config builder
//!
//! Shell configuration "wizard" with many options to configure any aspect of your shell. Start
//! with a [`ShellBuilder`], configure to your liking, finalize it, and then run the shell.

use std::{
    env,
    path::{Path, PathBuf},
    process::ExitStatus,
    time::Instant,
};

use anyhow::anyhow;
use dirs::home_dir;
use log::{info, warn};
use pino_deref::Deref;
use shrs_job::JobManager;

use crate::{
    commands::{Command, Commands},
    history::History,
    prelude::*,
    state::States,
};

/// Keeps track of how long the plugin initialization process took
#[derive(Deref)]
pub struct StartupTime(Instant);

/// Holds metadata for each plugin
#[derive(Deref)]
pub struct PluginMetas(Vec<PluginMeta>);

/// Container for shell components
///
/// This struct can be queried for in any handler (insert link), allowing for easy access to shell
/// components.
pub struct Shell {
    /// Builtin shell functions that have access to the shell's context
    pub builtins: Builtins,
    /// The command language
    pub lang: Box<dyn Lang>,
    /// Shell keybindings
    pub keybindings: Keybindings,
    /// Register new hooks to events or emit an event
    pub hooks: Hooks,
    /// Shell prompt
    pub prompt: Prompt,
    /// Syntax highlighter
    pub highlighter: Box<dyn Highlighter>,
    /// Active command suggestion
    pub suggester: Box<dyn Suggester>,
    /// Command history
    pub history: Box<dyn History>,
    cmd: Commands,
}

impl Shell {
    /// Run an arbitrary shell [`Command`]
    pub fn run_cmd<C: Command + 'static>(&self, command: C) {
        self.cmd.run(command);
    }

    /// Trigger an event of given type with payload.
    ///
    /// See [`crate::hooks`] for details.
    pub fn run_hooks<C: HookEventMarker>(&self, c: C) {
        self.cmd.run(move |sh: &mut Shell, states: &mut States| {
            let _ = sh.hooks.run(sh, states, &c);
            sh.apply_queue(states);
        })
    }

    pub(crate) fn run_hooks_in_core<C: HookEventMarker>(&mut self, states: &mut States, c: C) {
        let _ = self.hooks.run(self, states, &c);
        self.apply_queue(states);
    }

    /// Execute all the queued commands
    ///
    /// Commands that are queued via [`Shell::run_cmd`] are not executed until a call to this
    /// function is made.
    pub(crate) fn apply_queue(&mut self, states: &mut States) {
        let mut q = self.cmd.drain(states);
        while let Some(command) = q.pop_front() {
            command.apply(self, states);
        }
    }

    /// Evaluate an arbitrary command programatically using the shell interpreter
    ///
    /// The command will be evaluated as if the user has typed this string in the prompt
    /// themselves.
    pub fn eval(&self, cmd_str: impl ToString) {
        // TODO we can't actually get the result of this currently since it is queued
        let cmd_str = cmd_str.to_string();
        self.cmd.run(move |sh: &mut Shell, states: &mut States| {
            // TODO should handle this error?
            let _ = sh.lang.eval(sh, states, cmd_str.clone());
        });
    }
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
    /// Runtime hooks, see [`crate::hooks`]
    #[builder(default = "Hooks::default()")]
    pub hooks: Hooks,

    /// Builtin shell commands, see [`crate::builtin`]
    #[builder(default = "Builtins::default()")]
    pub builtins: Builtins,

    /// Readline implementation
    #[builder(default = "Box::new(Line::default())")]
    #[builder(setter(custom))]
    pub readline: Box<dyn Readline>,

    /// Aliases, see [`crate::alias`]
    #[builder(default = "Alias::new()")]
    pub alias: Alias,

    /// Environment variables, see [`crate::env`]
    #[builder(default = "Env::default()")]
    pub env: Env,

    /// Completion system, see [`crate::completion`]
    #[builder(default = "Box::new(DefaultCompleter::new())")]
    #[builder(setter(custom))]
    completer: Box<dyn Completer>,

    // /// List of defined functions
    // #[builder(default = "HashMap::new()")]
    // pub functions: HashMap<String, Box<ast::Command>>,
    /// Color theme, see [`crate::theme`]
    #[builder(default = "Theme::default()")]
    pub theme: Theme,

    /// Command language
    #[builder(default = "Box::new(PosixLang::default())")]
    #[builder(setter(custom))]
    pub lang: Box<dyn Lang>,

    /// Plugins, see [`crate::plugin`]
    #[builder(default = "Vec::new()")]
    #[builder(setter(custom))]
    pub plugins: Vec<Box<dyn Plugin>>, // TODO could also maybe use anymap to get the concrete type

    /// Globally accessible state, see [`crate::state`]
    #[builder(default = "States::default()")]
    #[builder(setter(custom))]
    pub states: States,

    /// History, see [`crate::history`]
    // The default is set again in DefaultHistoryPlugin so this default is just a dummy
    #[builder(default = "Box::new(DefaultHistory::default())")]
    #[builder(setter(custom))]
    pub history: Box<dyn History>,

    /// Configuration directory, easy access in the shell
    #[builder(default = "home_dir().unwrap().join(\".config/shrs\")")]
    pub config_dir: PathBuf,

    /// Keybindings, see [`crate::keybinding`]
    #[builder(default = "Keybindings::new()")]
    #[builder(setter(custom))]
    pub keybinding: Keybindings,

    //-------
    //Line
    /// Completion menu, see [`crate::readline::menu`]
    #[builder(default = "Box::new(DefaultMenu::default())")]
    #[builder(setter(custom))]
    menu: DefaultMenuState,

    /// Edit history tracked in readline buffer
    #[builder(default = "Box::new(DefaultBufferHistory::default())")]
    #[builder(setter(custom))]
    buffer_history: Box<dyn BufferHistory>,

    /// Syntax highlighter, see [`crate::readline::highlight`]
    #[builder(default = "Box::new(SyntaxHighlighter::default())")]
    #[builder(setter(custom))]
    highlighter: Box<dyn Highlighter>,

    /// Custom prompt, see [`crate::prompt`]
    #[builder(default = "Prompt::default()")]
    prompt: Prompt,

    /// Suggestion inline, see [`crate::readline::suggester`]
    #[builder(default = "Box::new(DefaultSuggester)")]
    suggester: Box<dyn Suggester>,

    /// Buffer snippets, see [`crate::readline::snippet`]
    #[builder(default = "Snippets::default()")]
    snippets: Snippets,
}

impl ShellBuilder {
    pub fn with_plugin<P: std::any::Any + Plugin>(mut self, plugin: P) -> Self {
        let mut cur_plugins = self.plugins.unwrap_or_default();
        cur_plugins.push(Box::new(plugin));
        self.plugins = Some(cur_plugins);

        self
    }
    pub fn with_state<T: 'static>(mut self, state: T) -> Self {
        let mut cur_states = self.states.unwrap_or_default();
        cur_states.insert(state);
        self.states = Some(cur_states);
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
    pub fn with_completer(mut self, completer: impl Completer + 'static) -> Self {
        self.completer = Some(Box::new(completer));
        self
    }
    pub fn with_keybindings(mut self, keybinding: Keybindings) -> Self {
        self.keybinding = Some(keybinding);
        self
    }
    pub fn with_menu(
        mut self,
        menu: impl Menu<MenuItem = Completion, PreviewItem = String> + 'static,
    ) -> Self {
        self.menu = Some(Box::new(menu));
        self
    }
    pub fn with_highlighter(mut self, highlighter: impl Highlighter + 'static) -> Self {
        self.highlighter = Some(Box::new(highlighter));
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
        let rt = Runtime {
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
        self.states.insert(rt);
        self.states.insert(self.alias);
        self.states.insert(OutputWriter::new(
            self.theme.out_style,
            self.theme.err_style,
        ));
        self.states.insert(self.theme);
        self.states.insert(Jobs::default());
        self.states.insert(PromptContentQueue::new());
        self.states.insert(self.completer);
        self.states.insert(StartupTime(Instant::now()));
        self.states.insert(PluginMetas(
            plugins
                .iter()
                .map(|p| p.meta())
                .collect::<Vec<PluginMeta>>(),
        ));
        self.states.insert(JobManager::default());

        //Line states
        self.states.insert(self.buffer_history);
        self.states.insert(self.menu);
        self.states.insert(self.snippets);

        let mut sh = Shell {
            builtins: self.builtins,
            lang: self.lang,
            keybindings: self.keybinding,
            hooks: self.hooks,
            prompt: self.prompt,
            highlighter: self.highlighter,
            suggester: self.suggester,
            history: self.history,
            cmd: Commands::new(),
        };

        // run post init for plugins
        for plugin in plugins.iter() {
            if let Err(e) = plugin.post_init(&mut sh, &mut self.states) {
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

        run_shell(&mut self.states, &mut sh, &mut self.readline)
    }
}

fn run_shell(
    states: &mut States,
    sh: &mut Shell,
    readline: &mut Box<dyn Readline>,
) -> anyhow::Result<()> {
    // init stuff
    let startup_ctx = StartupCtx {
        startup_time: states.get::<StartupTime>().elapsed(),
    };

    sh.run_hooks_in_core(states, startup_ctx);

    loop {
        let line = readline.read_line(sh, states);

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
                states,
            };

            // Currently only use the last alias, can also render a menu
            if let Some(expanded) = states.get::<Alias>().get(&alias_ctx).last() {
                *first = expanded.to_string();
            }
        }
        let line = words.join(" ");

        // TODO not sure if hook should run here (since not all vars are expanded yet)
        let hook_ctx = BeforeCommandCtx {
            raw_command: line.clone(),
            command: line.clone(),
        };
        sh.run_hooks_in_core(states, hook_ctx);

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
        states.get_mut::<OutputWriter>().begin_collecting();
        if let Some(builtin_cmd) = builtin_cmd {
            let output = builtin_cmd.run(sh, states, &words);
            match output {
                Ok(o) => cmd_output = o,
                Err(e) => eprintln!("error: {e:?}"),
            }

            sh.apply_queue(states);
        } else {
            let output = sh.lang.eval(sh, states, line.clone());
            match output {
                Ok(o) => cmd_output = o,
                Err(e) => eprintln!("error: {e:?}"),
            }
        }
        let (out, err) = states.get_mut::<OutputWriter>().end_collecting();
        cmd_output.stdout(out);
        cmd_output.stderr(err);

        sh.run_hooks_in_core(
            states,
            AfterCommandCtx {
                command: line,
                cmd_output,
            },
        );

        // check up on running jobs
        let mut exit_statuses = vec![];
        states.get_mut::<Jobs>().retain(|status: ExitStatus| {
            exit_statuses.push(status);
        });

        sh.run_hooks_in_core(states, JobExitCtx { exit_statuses });
    }
}

/// Set the current working directory programatically
///
/// The `run_hook` parameter determines if a change directory event should be emitted.
pub fn set_working_dir(
    sh: &Shell,
    rt: &mut StateMut<Runtime>,
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
    let old_path = get_working_dir(&rt).to_path_buf();
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
            old_dir: old_path.clone(),
            new_dir: path.clone(),
        };
        sh.run_hooks(hook_ctx);
    }

    Ok(())
}

/// Fetch the current working directory the shell is in
pub fn get_working_dir(rt: &Runtime) -> &Path {
    &rt.working_dir
}
