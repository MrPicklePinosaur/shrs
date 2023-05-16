//! Save and load shell run context
//!
//!

mod builtin;

use std::{
    collections::HashMap,
    io::BufWriter,
    marker::PhantomData,
    time::{Duration, Instant},
};

use builtin::{LoadBuiltin, SaveBuiltin};
use shrs::{
    anyhow,
    hooks::{AfterCommandCtx, BeforeCommandCtx},
    plugin::{Plugin, ShellPlugin},
    Context, Runtime, Shell,
};

pub struct RunContextState {
    pub(crate) run_contexts: HashMap<String, Runtime>,
}

impl RunContextState {
    pub fn new() -> Self {
        Self {
            run_contexts: HashMap::new(),
        }
    }
}

pub struct RunContextPlugin;

impl Plugin for RunContextPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.builtins.insert("save", SaveBuiltin);
        shell.builtins.insert("load", LoadBuiltin);
        shell.state.insert(RunContextState::new());
    }
}
