use std::{
    cell::RefCell,
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
    fn apply(&self, sh: &mut Shell, states: &States);
}

impl<F> Command for F
where
    F: Fn(&mut Shell, &States) + Send + 'static,
{
    fn apply(&self, sh: &mut Shell, states: &States) {
        self(sh, states);
    }
}

pub struct Commands {
    pub queue: RefCell<VecDeque<Box<dyn Command>>>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands {
            queue: RefCell::new(VecDeque::new()),
        }
    }

    pub fn run<C: Command + 'static>(&self, command: C) {
        self.queue.borrow_mut().push_back(Box::new(command));
    }

    pub fn drain(&self, states: &States) -> VecDeque<Box<dyn Command>> {
        self.queue.borrow_mut().drain(..).collect()
    }

    // Evaluate an arbitrary command using the shell interpreter
    pub fn eval(&self, cmd_str: impl ToString) {
        // TODO we can't actually get the result of this currently since it is queued
        let cmd_str = cmd_str.to_string();
        self.run(move |sh: &mut Shell, states: &States| {
            // TODO should handle this error?
            let _ = sh.lang.eval(sh, states, cmd_str.clone());
        });
    }

    // Trigger a hook of given type with payload
    pub fn run_hook<C: HookCtx>(&self, hook_ctx: C) {
        self.run(move |sh: &mut Shell, states: &States| {
            let _ = sh.run_hooks(states, &hook_ctx);
        })
    }
}
