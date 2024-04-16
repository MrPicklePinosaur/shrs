//! Plugin System

use log::warn;

use crate::{
    prelude::ShellConfig,
    shell::{Context, Runtime, Shell},
};

#[derive(Debug)]
pub struct PluginMeta {
    pub name: String,
    pub description: String,
    pub help: Option<String>,
}
impl PluginMeta {
    pub fn new<S: ToString>(name: S, description: S, help: Option<S>) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            help: help.map(|s| s.to_string()),
        }
    }
}

/// How should the plugin be handled if it errors during initialization
#[derive(Debug)]
pub enum FailMode {
    /// Display a warning but continue with shell initialization
    Warn,
    /// Abort entire shell initialization process and crash
    Abort,
}

impl Default for PluginMeta {
    fn default() -> Self {
        Self {
            name: String::from("unnamed plugin"),
            description: String::from("a plugin for shrs"),
            help: None,
        }
    }
}

/// Implement this trait to build your own plugins
pub trait Plugin {
    /// Plugin initialization
    ///
    /// Hook onto the initialization of the shell and add any hooks, functions, state variables
    /// that you would like
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()>;

    /// Plugin post initialization
    ///
    /// Gets called once after the shell has completed initialization process, giving access to
    /// shell, context, and runtime state
    fn post_init(&self, _sh: &Shell, _ctx: &mut Context, _rt: &mut Runtime) -> anyhow::Result<()> {
        Ok(())
    }

    /// Return metadata related to the plugin
    fn meta(&self) -> PluginMeta {
        // TODO this is currently an optional method to make migrating all the existing plugins a
        // bit easier. Could remove the default implementation in the future
        warn!("Using default plugin metadata. Please specify this information for your plugin by implementing Plugin::meta()");
        PluginMeta::default()
    }

    /// Get the fail mode for this plugin
    ///
    /// Provide implementation for this if you want non-default behavior
    fn fail_mode(&self) -> FailMode {
        // Default to more strict fail mode to let users know faster there's a bug
        //
        // Should consider more how good of an idea this is
        FailMode::Abort
    }
}

/// Extension trait to make [ShellConfig] support plugins
pub trait ShellPlugin {
    fn with_plugin(&mut self, plugin: impl Plugin);
}
