//! Plugin System
//!
//! Plugins is a mechanism for third parties to bundle up custom functionality and distribute them
//! to other users in a way that is easily installable and configurable. Generally, plugins are
//! published as rust crates, so the user will simply include your plugin as a dependency.
//!
//! To use a plugin, you include it in [`ShellConfig`] when constructing the shell.
//! ```ignore
//! use shrs_hello_plugin::HelloPlugin;
//!
//! // Do any initialization/configuration the plugin provides
//! let hello_plugin = HelloPlugin::new();
//!
//! // Using the plugin is as easy as:
//! let myshell = ShellBuilder::default().with_plugin(hello_plugin);
//! ```
//!
//! To develop your own plugins, it's as easy as implementing the [`Plugin`] trait. The [`Plugin`]
//! trait has [`Plugin::init()`] and [`Plugin::post_init()`] methods, which allows your plugin to
//! hook into the shell initialization process, and insert additional state like custom builtins,
//! keybindings, state, hooks and much more.
//! ```
//! # use shrs_core::prelude::*;
//! # #[derive(HookEvent)]
//! # struct MyHookEvent {}
//! // Define a struct for your plugin with any configuration as it's fields
//! pub struct MyPlugin {
//!     number: u32,
//! }
//!
//! impl MyPlugin {
//!     // A common pattern is to expose some constructor to allow the user to configure the plugin
//!     pub fn new(number: u32) -> Self {
//!         Self {
//!             number
//!         }
//!     }
//! }
//!
//! impl Plugin for MyPlugin {
//!
//!     fn init(&self, config: &mut ShellConfig) -> anyhow::Result<()> {
//!         # let my_hook = |event: &MyHookEvent| -> anyhow::Result<()> {
//!         #     Ok(())
//!         # };
//!         #
//!         # let my_state = ();
//!         // Insert any state here
//!         config.hooks.insert(my_hook);
//!         config.states.insert(my_state);
//!         Ok(())
//!     }
//!
//!     fn meta(&self) -> PluginMeta {
//!         PluginMeta {
//!             name: "MyPlugin".into(),
//!             description: "My demo plugin".into(),
//!             help: None,
//!         }
//!     }
//! }
//! ```

use log::warn;

use crate::prelude::{Shell, ShellConfig, States};

/// Metadata for your plugin
#[derive(Debug)]
pub struct PluginMeta {
    /// Name of the plugin
    pub name: String,
    /// Brief description on what the plugin does
    pub description: String,
    /// Optional help message to be used by the help builtin
    pub help: Option<String>,
}

impl PluginMeta {
    /// Construct a new plugin meta data
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
    fn init(&self, config: &mut ShellConfig) -> anyhow::Result<()>;

    /// Plugin post initialization
    ///
    /// Gets called once after the shell has completed initialization process, giving access to
    /// runtime shells state. This should be used if you depend on shell other state.
    fn post_init(&self, _sh: &mut Shell, _states: &mut States) -> anyhow::Result<()> {
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
    /// Provide implementation for this if you want non-default behavior. See [`FailMode`].
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
