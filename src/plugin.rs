//! Plugin System

use crate::ShellConfig;

/// Implement this trait to build your own plugins
pub trait Plugin {
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
