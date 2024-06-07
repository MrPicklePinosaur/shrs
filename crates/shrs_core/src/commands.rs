//! Command queue for mutating `States` and `Shell` in handlers
//!
//! Commands can be pushed into a queue in handlers
//! and will be run directly after it ends.
//! `Commands` is stored in `Shell`, which can be accessed in handlers.
//! ```rust
//! # use shrs_core::prelude::*;
//! pub struct C {}
//! fn add_state(sh: &Shell){
//!    sh.run_cmd(|sh: &mut Shell, states: &mut States| {
//!        states.insert(C {});
//!    });
//! }
//! ```
//! Functions with the correct signature automatically implement `Command`.

use std::{cell::RefCell, collections::VecDeque};

use crate::{prelude::Shell, state::States};

/// Functions that can mutate `Shell` and `State`
///
/// Trait automatically implemented for functions of the correct signature.
/// Runs after handler completes
pub trait Command: Send + 'static {
    fn apply(self: Box<Self>, sh: &mut Shell, states: &mut States);
}

impl<F> Command for F
where
    F: FnOnce(&mut Shell, &mut States) + Send + 'static,
{
    fn apply(self: Box<Self>, sh: &mut Shell, states: &mut States) {
        self(sh, states);
    }
}

pub struct Commands {
    queue: RefCell<VecDeque<Box<dyn Command>>>,
}

impl Commands {
    pub(crate) fn new() -> Commands {
        Commands {
            queue: RefCell::new(VecDeque::new()),
        }
    }

    pub(crate) fn run<C: Command + 'static>(&self, command: C) {
        self.queue.borrow_mut().push_back(Box::new(command));
    }

    pub(crate) fn drain(&self, states: &States) -> VecDeque<Box<dyn Command>> {
        self.queue.borrow_mut().drain(..).collect()
    }
}
