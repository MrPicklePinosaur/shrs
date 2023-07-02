//! Plugin System

use log::warn;

use crate::ShellConfig;

#[derive(Debug)]
pub struct PluginMeta {
    pub name: String,
    pub description: String,
}

impl Default for PluginMeta {
    fn default() -> Self {
        Self {
            name: String::from("unnamed plugin"),
            description: String::from("a plugin for shrs"),
        }
    }
}

/// Implement this trait to build your own plugins
pub trait Plugin {
    /// Return metadata related to the plugin
    fn meta(&self) -> PluginMeta {
        // TODO this is currently an optional method to make migrating all the existing plugins a
        // bit easier. Could remove the default implementation in the future
        warn!("Using default plugin metadata. Please specify this information for your plugin by implmenting Plugin::meta()");
        PluginMeta::default()
    }
    /// Plugin entry point
    ///
    /// Hook onto the initialization of the shell and add any hooks, functions, state variables
    /// that you would like
    fn init(&self, shell: &mut ShellConfig);
}

/// Extension trait to make [ShellConfig] support plugins
pub trait ShellPlugin {
    fn with_plugin(&mut self, plugin: impl Plugin);
}
