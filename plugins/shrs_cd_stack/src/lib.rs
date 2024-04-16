use std::{
    collections::LinkedList,
    env::current_dir,
    path::{Path, PathBuf},
};

use shrs::prelude::*;

/// Resembles undo/redo but for file path histories
pub struct CdStackState {
    down_stack: LinkedList<PathBuf>,
    /// Used to go back up path history if path history was not changed
    up_stack: LinkedList<PathBuf>,
}

impl CdStackState {
    pub fn new() -> Self {
        Self {
            down_stack: LinkedList::new(),
            up_stack: LinkedList::new(),
        }
    }

    /// Add new directory location to path
    pub fn push(&mut self, path: &Path) {
        self.down_stack.push_back(path.to_path_buf());
        self.up_stack.clear();
    }

    /// Go back in path history
    pub fn down(&mut self) -> Option<PathBuf> {
        let top = self.down_stack.pop_back();
        if let Some(top) = top {
            self.up_stack.push_back(top);
        }
        self.down_stack.back().cloned()
    }

    /// Go forward in path history
    pub fn up(&mut self) -> Option<PathBuf> {
        let top = self.up_stack.pop_back();
        if let Some(top) = top.clone() {
            self.down_stack.push_back(top);
        }
        top
    }
}

fn change_dir_hook(
    _sh: &Shell,
    ctx: &mut Context,
    _rt: &mut Runtime,
    hook_ctx: &ChangeDirCtx,
) -> anyhow::Result<()> {
    if let Some(state) = ctx.state.get_mut::<CdStackState>() {
        state.push(&hook_ctx.new_dir);
    }
    Ok(())
}

pub struct CdStackPlugin;

impl Plugin for CdStackPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        let mut cd_stack_state = CdStackState::new();
        // TODO hopefully would be better to get current dir from shell, but shell isn't
        // constructed yet
        cd_stack_state.push(&current_dir().unwrap());
        shell.state.insert(cd_stack_state);
        shell.hooks.insert(change_dir_hook);

        Ok(())
    }
    fn meta(&self) -> PluginMeta {
        PluginMeta::new(
            "Cd Stack",
            "Provides the ability to quickly navigate directories like a stack.",
            None,
        )
    }
}
