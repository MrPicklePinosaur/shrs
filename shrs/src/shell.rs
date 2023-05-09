use std::{
    collections::HashMap,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    time::Instant,
};

use shrs_core::{
    builtin::Builtins,
    hooks::{AfterCommandCtx, BeforeCommandCtx, Hooks, JobExitCtx, StartupCtx},
    Alias, Context, Env, Jobs, Runtime, Shell, State, Theme,
};
use shrs_lang::ast;
use shrs_line::{DefaultHistory, DefaultPrompt, History, Line, Prompt};

use crate::plugin::Plugin;

/// Unified shell config struct
#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(setter(prefix = "with"))]
pub struct ShellConfig {
    #[builder(default = "Hooks::default()")]
    pub hooks: Hooks,

    #[builder(default = "Builtins::default()")]
    pub builtins: Builtins,

    #[builder(default = "Line::default()")]
    pub readline: Line,

    #[builder(default = "Box::new(DefaultHistory::new())")]
    #[builder(setter(custom))]
    pub history: Box<dyn History<HistoryItem = String>>,

    #[builder(default = "Alias::new()")]
    pub alias: Alias,

    /// Custom prompt
    #[builder(default = "Box::new(DefaultPrompt::new())")]
    #[builder(setter(custom))]
    pub prompt: Box<dyn Prompt>,

    /// Environment variables
    #[builder(default = "Env::new()")]
    pub env: Env,

    /// List of defined functions
    #[builder(default = "HashMap::new()")]
    pub functions: HashMap<String, Box<ast::Command>>,

    /// Color theme
    #[builder(default = "Theme::default()")]
    pub theme: Theme,

    /// Plugins
    #[builder(default = "Vec::new()")]
    #[builder(setter(custom))]
    pub plugins: Vec<Box<dyn Plugin>>,

    /// Globally accessable state
    #[builder(default = "State::new()")]
    #[builder(setter(custom))]
    pub state: State,
}

impl ShellConfigBuilder {
    pub fn with_prompt(mut self, prompt: impl Prompt + 'static) -> Self {
        self.prompt = Some(Box::new(prompt));
        self
    }
    pub fn with_history(mut self, history: impl History<HistoryItem = String> + 'static) -> Self {
        self.history = Some(Box::new(history));
        self
    }
    pub fn with_plugin(mut self, plugin: impl Plugin + 'static) -> Self {
        let mut cur_plugin = self.plugins.unwrap_or(vec![]);
        cur_plugin.push(Box::new(plugin));
        self.plugins = Some(cur_plugin);
        self
    }
    pub fn with_state<T: 'static>(mut self, state: T) -> Self {
        let mut cur_state = self.state.unwrap_or(State::new());
        cur_state.insert(state);
        self.state = Some(cur_state);
        self
    }
}

impl ShellConfig {
    pub fn run(mut self) -> anyhow::Result<()> {
        // TODO some default values for Context and Runtime are duplicated by the #[builder(default = "...")]
        // calls in ShellConfigBuilder, so we are sort of defining the full default here. Maybe end
        // up implementing Default for Context and Runtime

        // run plugins first
        let plugins = self.plugins.drain(..).collect::<Vec<_>>();
        for plugin in plugins {
            plugin.init(&mut self);
        }

        let mut ctx = Context {
            readline: self.readline,
            history: self.history,
            alias: self.alias,
            prompt: self.prompt,
            out: BufWriter::new(stdout()),
            state: self.state,
            jobs: Jobs::new(),
        };
        let mut rt = Runtime {
            env: self.env,
            working_dir: std::env::current_dir().unwrap(),
            // TODO currently hardcoded
            name: "shrs".into(),
            // TODO currently unused (since we have not implemented functions etc)
            args: vec![],
            exit_status: 0,
            functions: self.functions,
            startup_time: Instant::now(),
        };
        let shell = Shell {
            builtins: self.builtins,
            hooks: self.hooks,
            theme: self.theme,
        };

        shell.run(&mut ctx, &mut rt)
    }
}
