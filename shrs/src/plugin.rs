//! Plugin System

use crate::{ShellConfig, ShellConfigBuilder};

pub trait Plugin {
    fn init(&self, shell: &mut ShellConfig);
}

/// Extension trait to make [ShellConfig] support plugins
pub trait ShellPlugin {
    fn with_plugin(&mut self, plugin: impl Plugin);
}
