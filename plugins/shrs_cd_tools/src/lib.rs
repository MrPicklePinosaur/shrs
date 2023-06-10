//! Variety of utilities for running commands conditionally on directory change
//!
//!

#[macro_use]
extern crate derive_builder;

pub mod git;
pub mod query;
pub mod rust;

use std::collections::HashMap;

use anymap::AnyMap;
use query::{Query, QueryResult};
use shrs::prelude::*;

pub struct DirParsePlugin {
    // pub modules: Option<Vec<Query>>,
}

impl DirParsePlugin {
    // pub fn new(modules: Vec<Query>) -> Self {
    pub fn new() -> Self {
        Self {
            // modules: Some(modules)
        }
    }
}

pub struct DirParseState {
    pub modules: HashMap<String, Query>,
    pub(crate) parsed_modules: HashMap<String, QueryResult>,
}

impl DirParseState {
    pub fn new(modules: HashMap<String, Query>) -> Self {
        Self {
            modules,
            parsed_modules: HashMap::new(),
        }
    }

    // /// Set the value of a module
    // pub(crate) fn set_parsed_module(&mut self, module: String, data: QueryResult) {
    //     self.parsed_modules.insert(module, data);
    // }

    pub fn get_module(&self, module: &str) -> Option<&QueryResult> {
        self.parsed_modules.get(module)
    }
}

pub fn change_dir_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &ChangeDirCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<DirParseState>() {
        // TODO this code is horribly inefficient lol
        let mut updated: HashMap<String, QueryResult> = HashMap::new();
        for (mod_name, module) in state.modules.iter() {
            let query_res = module.scan(&sh_rt.working_dir);
            updated.insert(mod_name.to_string(), query_res);
        }
        state.parsed_modules = updated;
    }
    Ok(())
}

impl Plugin for DirParsePlugin {
    fn init(&self, shell: &mut ShellConfig) {
        // TODO let user pass in their own modules list
        let modules = HashMap::from_iter([(String::from("rust"), rust::module().unwrap())]);

        shell.state.insert(DirParseState::new(modules));
        shell.hooks.register(change_dir_hook);
    }
}
