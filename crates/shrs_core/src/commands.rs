use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use log::warn;

use crate::{
    prelude::{HookCtx, Shell},
    state::States,
};

pub trait Command: Send + 'static {
    fn apply(&self, sh: &mut Shell, states: &States, cmd: &mut Commands);
}

impl<F> Command for F
where
    F: Fn(&mut Shell, &States, &mut Commands) + Send + 'static,
{
    fn apply(&self, sh: &mut Shell, states: &States, cmd: &mut Commands) {
        self(sh, states, cmd);
    }
}

pub struct Commands {
    pub queue: VecDeque<Box<dyn Command>>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            queue: VecDeque::new(),
        }
    }

    pub fn run<C: Command + 'static>(&mut self, command: C) {
        self.queue.push_back(Box::new(command));
    }

    pub fn apply_all(&mut self, sh: &mut Shell, states: &States) -> VecDeque<Box<dyn Command>> {
        let mut to_run = VecDeque::new();
        while let Some(cmd) = self.queue.pop_front() {
            to_run.push_back(cmd);
        }
        to_run
    }

    // Evaluate an arbitrary command using the shell interpreter
    pub fn eval(&mut self, cmd_str: impl ToString) {
        // TODO we can't actually get the result of this currently since it is queued
        let cmd_str = cmd_str.to_string();
        self.run(move |sh: &mut Shell, states: &States, cmd: &mut Commands| {
            // TODO should handle this error?
            let _ = sh.lang.eval(sh, states, cmd_str.clone());
        });
    }

    // Trigger a hook of given type with payload
    // pub fn run_hook<C: HookCtx>(&mut self, hook_ctx: C) {
    //     self.run(move |sh: &mut Shell, states: &States| {
    //         let _ = sh.run_hooks(states, &hook_ctx);
    //     })
    // }
}
