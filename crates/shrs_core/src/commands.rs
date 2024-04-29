use std::collections::VecDeque;

use log::warn;

use crate::{prelude::{Shell, HookCtx}, state::States};

pub trait Command: Send + 'static {
    fn apply(&self, sh: &mut Shell, states: &States);
}
impl<F> Command for F
where
    F: Fn(&mut Shell, &States) + Send + 'static,
{
    fn apply(&self, sh: &mut Shell, states: &States) {
        dbg!(1);
        self(sh, states);
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
    pub fn eval(&mut self, cmd: impl ToString) {
        // TODO we can't actually get the result of this currently since it is queued
        let cmd = cmd.to_string();
        self.run(move |sh: &mut Shell, states: &States| {
            // TODO should handle this error?
            let _ = sh.lang.eval(sh, states, cmd.clone());
        });
    }

    // Trigger a hook of given type with payload
    pub fn run_hook<C: HookCtx>(&mut self, hook_ctx: C) {
        self.run(move |sh: &mut Shell, states: &States| {
            if let Err(e) = sh.hooks.run(sh, states, hook_ctx.clone()) {
                let type_name = std::any::type_name::<C>();
                warn!("failed to execute hook {e} of type {type_name}");
            }
        })
    }

}
