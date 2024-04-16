//! Variety of utilities for running commands conditionally on directory change
//!
//!

#[macro_use]
extern crate derive_builder;

pub mod git;
pub mod node;
pub mod query;
pub mod rust;

use std::collections::HashMap;

use query::{Query, QueryResult};
use shrs::prelude::{styled_buf::StyledBuf, *};

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

    pub fn get_module_metadata<T: 'static>(&self, module: &str) -> Option<&T> {
        self.get_module(module)
            .and_then(|module| module.get_metadata::<T>())
    }
}

pub fn startup_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    _ctx: &StartupCtx,
) -> anyhow::Result<()> {
    update_modules(sh_ctx, sh_rt)?;
    Ok(())
}

pub fn change_dir_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    _ctx: &ChangeDirCtx,
) -> anyhow::Result<()> {
    update_modules(sh_ctx, sh_rt)?;
    Ok(())
}

fn update_modules(sh_ctx: &mut Context, sh_rt: &mut Runtime) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<DirParseState>() {
        // TODO this code is horribly inefficient lol
        let mut updated: HashMap<String, QueryResult> = HashMap::new();
        for (mod_name, module) in state.modules.iter() {
            let mut query_res = module.scan(&sh_rt.working_dir);
            if query_res.matched {
                // NOTE we ignore errors in metadata fn
                let _ = module.metadata_fn(&mut query_res);
                updated.insert(mod_name.to_string(), query_res);
            }
        }
        state.parsed_modules = updated;
    }
    Ok(())
}

impl Plugin for DirParsePlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        // TODO let user pass in their own modules list
        let modules = HashMap::from_iter([
            (String::from("rust"), rust::module().unwrap()),
            (String::from("node"), node::module().unwrap()),
            (String::from("git"), git::module().unwrap()),
        ]);

        shell.state.insert(DirParseState::new(modules));
        shell.hooks.insert(startup_hook);
        shell.hooks.insert(change_dir_hook);

        Ok(())
    }
}

/// Default example prompt that displays some information based on language
pub fn default_prompt(line_ctx: &LineStateBundle) -> StyledBuf {
    if let Some(dir_parse_state) = line_ctx.ctx.state.get::<DirParseState>() {
        let rust_info: Option<String> = dir_parse_state
            .get_module_metadata::<rust::CargoToml>("rust")
            .map(|cargo_toml| format!("🦀 {} ", cargo_toml.package.edition));

        let node_info: Option<String> = dir_parse_state
            .get_module_metadata::<node::NodeJs>("node")
            .map(|node_js| format!(" {} ", node_js.version));

        styled_buf! {
            rust_info, node_info,
        }
    } else {
        styled_buf! {
            ""
        }
    }
}
