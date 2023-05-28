use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{stdout, BufRead, BufWriter, Write},
    path::Path,
    process::{Child, Stdio},
    time::Instant,
};

use lazy_static::lazy_static;
use shrs_core::{
    builtin::Builtins,
    dummy_child,
    hooks::{BeforeCommandCtx, Hooks, JobExitCtx, StartupCtx},
    Alias, Context, Env, ExitStatus, Jobs, Lang, Runtime, Shell, Signals, State, Theme,
};
use shrs_lang::PosixLang;
use shrs_line::{DefaultPrompt, Line, Prompt};
use thiserror::Error;

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

    #[builder(default = "Alias::new()")]
    pub alias: Alias,

    /// Environment variables
    #[builder(default = "Env::new()")]
    pub env: Env,

    // /// List of defined functions
    // #[builder(default = "HashMap::new()")]
    // pub functions: HashMap<String, Box<ast::Command>>,
    /// Color theme
    #[builder(default = "Theme::default()")]
    pub theme: Theme,

    /// Command language
    #[builder(default = "Box::new(PosixLang::new())")]
    #[builder(setter(custom))]
    pub lang: Box<dyn Lang>,

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
    pub fn with_lang(mut self, lang: impl Lang + 'static) -> Self {
        self.lang = Some(Box::new(lang));
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
            alias: self.alias,
            out: BufWriter::new(stdout()),
            state: self.state,
            jobs: Jobs::new(),
            startup_time: Instant::now(),
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
            builtins: self.builtins,
            theme: self.theme,
            lang: self.lang,
            hooks: self.hooks,
            signals: Signals::new().unwrap(),
        };
        let mut readline = self.readline;

        run_shell(&sh, &mut ctx, &mut rt, &mut readline)
    }
}

fn run_shell(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    readline: &mut Line,
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
            if let Some(expanded) = ctx.alias.get(&first) {
                *first = expanded.to_owned().to_string();
            }
        }
        let line = words.join(" ");

        // TODO not sure if hook should run here (since not all vars are expanded yet)
        let hook_ctx = BeforeCommandCtx {
            raw_command: line.clone(),
            command: line.clone(),
        };
        sh.hooks.run::<BeforeCommandCtx>(sh, ctx, rt, hook_ctx)?;

        match sh.lang.eval(sh, ctx, rt, line) {
            Ok(_) => {},
            Err(_) => {},
        }

        // check up on running jobs
        let mut exit_statuses = vec![];
        ctx.jobs.retain(|status: ExitStatus| {
            exit_statuses.push(status);
        });

        for status in exit_statuses.into_iter() {
            sh.hooks
                .run::<JobExitCtx>(sh, ctx, rt, JobExitCtx { status });
        }
    }
}
