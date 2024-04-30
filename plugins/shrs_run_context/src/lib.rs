//! Save and load shell run context
//!
//!

mod builtin;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use builtin::{load_builtin, save_builtin};
use shrs::prelude::*;

pub struct RunContextState {
    pub(crate) run_contexts: HashMap<String, Runtime>,
    pub(crate) context_file: Option<PathBuf>,
}

#[derive(Default)]
pub struct RunContextPlugin {
    context_file: Option<PathBuf>,
}

impl RunContextPlugin {
    pub fn with_file(path: &Path) -> Self {
        RunContextPlugin {
            context_file: Some(path.to_owned()),
        }
    }
}

impl Plugin for RunContextPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta::new(
            "Run Context",
            "Provides commands for storing the current run context and loading it",
            None,
        )
    }

    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.builtins.insert("save", save_builtin);
        shell.builtins.insert("load", load_builtin);

        // if context file was provided, read file into context state
        if let Some(context_file) = &self.context_file {
            // TODO create file if not exist

            let contents = fs::read_to_string(context_file)?;
            let run_contexts: HashMap<String, Runtime> = ron::from_str(&contents)?;

            shell.states.insert(RunContextState {
                run_contexts,
                context_file: Some(context_file.clone()),
            });
        } else {
            shell.states.insert(RunContextState {
                run_contexts: HashMap::new(),
                context_file: None,
            });
        }

        Ok(())
    }
}
