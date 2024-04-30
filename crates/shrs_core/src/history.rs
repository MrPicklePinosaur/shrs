use std::cell::RefCell;

use crate::prelude::{Shell, States};

/// Trait to implement for shell history
pub trait History {
    /// Insert cmd into shell history
    fn add(&self, sh: &Shell, states: &States, cmd: String);
    /// Remove all history entries
    fn clear(&self, sh: &Shell, states: &States);
    /// Get number of history entries
    fn len(&self, sh: &Shell, states: &States) -> usize;
    /// Get a history entry by index
    fn get(&self, sh: &Shell, states: &States, i: usize) -> Option<String>;

    /// Check if the history is empty
    fn is_empty(&self, sh: &Shell, states: &States) -> bool {
        self.len(sh, states) == 0
    }
    fn items(&self, sh: &Shell, states: &States) -> Vec<String>;
}
/// Default implementation of [History] that saves history in process memory
#[derive(Default)]
pub struct DefaultHistory {
    hist: RefCell<Vec<String>>,
}

impl History for DefaultHistory {
    fn add(&self, sh: &Shell, states: &States, cmd: String) {
        if !cmd.starts_with("history run") {
            self.hist.borrow_mut().insert(0, cmd);
        }
    }

    fn clear(&self, sh: &Shell, states: &States) {
        self.hist.borrow_mut().clear();
    }

    fn len(&self, sh: &Shell, states: &States) -> usize {
        self.hist.borrow().len()
    }
    /// Get index starts at most recent (index zero is previous command)

    fn get(&self, sh: &Shell, states: &States, i: usize) -> Option<String> {
        self.hist.borrow().get(i).cloned()
    }
    fn items(&self, sh: &Shell, states: &States) -> Vec<String> {
        self.hist.borrow().clone()
    }
}
