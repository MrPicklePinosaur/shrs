//! Plugin System

use log::warn;

use crate::ShellConfig;

#[derive(Debug)]
pub struct PluginMeta {
    pub name: String,   // should be unique
    pub description: String,
}

/// How should the plugin be handled if it errors during initialization
#[derive(Debug)]
pub enum FailMode {
    /// Display a warning but continue with shell initialization
    Warn,
    /// Abort entire shell initialization process and crash
    Abort,
}

/// Implement this trait to build your own plugins
pub trait Plugin {
    /// Plugin entry point
    ///
    /// Hook onto the initialization of the shell and add any hooks, functions, state variables
    /// that you would like
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()>;

    /// Return metadata related to the plugin
    fn meta(&self) -> PluginMeta;

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
