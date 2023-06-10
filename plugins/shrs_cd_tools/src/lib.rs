//! Variety of utilities for running commands conditionally on directory change
//!
//!

#[macro_use]
extern crate derive_builder;

pub mod git;
pub mod query;
pub mod rust;

use anymap::AnyMap;
use query::Query;
use shrs::prelude::*;

pub struct DirParsePlugin {
    // pub modules: Option<Vec<Query>>,
}

impl DirParsePlugin {
    pub fn new(modules: Vec<Query>) -> Self {
        Self {
            // modules: Some(modules)
        }
    }
}

pub struct DirParseState {
    pub modules: Vec<Query>,
    parsed_modules: AnyMap,
}

impl DirParseState {
    pub fn new(modules: Vec<Query>) -> Self {
        Self {
            modules,
            parsed_modules: AnyMap::new(),
        }
    }

    /// Set the value of a module
    ///
    /// In the case that a none is passed in, the data is instead deleted
    pub(crate) fn set_parsed_module<T: 'static>(&mut self, data: Option<T>) {
        match data {
            Some(inner) => {
                self.parsed_modules.insert(inner);
            },
            None => {
                self.parsed_modules.remove::<T>();
            },
        };
    }
}

pub fn change_dir_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &ChangeDirCtx,
) -> anyhow::Result<()> {
    Ok(())
}

impl Plugin for DirParsePlugin {
    fn init(&self, shell: &mut ShellConfig) {
        // TODO let user pass in their own modules list
        let modules = vec![rust::module().unwrap()];

        shell.state.insert(DirParseState::new(modules));
    }
}
