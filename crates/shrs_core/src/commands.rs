use std::collections::VecDeque;

use crate::{prelude::Shell, state::States};

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
    pub fn add<C: Command + 'static>(&mut self, command: C) {
        self.queue.push_back(Box::new(command));
    }
    pub fn apply_all(&mut self, sh: &mut Shell, states: &States) -> VecDeque<Box<dyn Command>> {
        let mut to_run = VecDeque::new();
        while let Some(cmd) = self.queue.pop_front() {
            to_run.push_back(cmd);
        }
        to_run
    }

    pub fn new() -> Commands {
        Commands {
            queue: VecDeque::new(),
        }
    }
}
