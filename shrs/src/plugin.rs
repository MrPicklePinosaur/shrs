//! Plugin System

use crate::ShellConfig;

pub trait Plugin {
    fn init(&self, shell: &mut ShellConfig);
}

/// Extension trait to make [ShellConfig] support plugins
pub trait ShellPlugin {
    fn with_plugin(&mut self, plugin: impl Plugin);
}

impl ShellPlugin for ShellConfig {
    fn with_plugin(&mut self, plugin: impl Plugin) {
        plugin.init(self);
    }
}
