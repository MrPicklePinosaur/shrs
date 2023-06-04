use std::{
    collections::{HashMap, HashSet, LinkedList},
    path::{Path, PathBuf},
};

use shrs::prelude::*;

pub struct CdStackState {
    dir_stack: LinkedList<PathBuf>,
}

impl CdStackState {
    pub fn new() -> Self {
        Self {
            dir_stack: LinkedList::new(),
        }
    }

    pub fn push(&mut self, path: &Path) {
        self.dir_stack.push_back(path.to_path_buf());
    }

    pub fn pop(&mut self) -> Option<PathBuf> {
        self.dir_stack.pop_back()
    }
}

fn change_dir_hook(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    hook_ctx: &ChangeDirCtx,
) -> anyhow::Result<()> {
    if let Some(state) = ctx.state.get_mut::<CdStackState>() {
        state.push(&hook_ctx.new_dir);
    }
    Ok(())
}

pub struct CdStackPlugin;

impl Plugin for CdStackPlugin {
    fn init(&self, shell: &mut ShellConfig) {
        shell.state.insert(CdStackState::new());
        shell.hooks.register(change_dir_hook);
    }
}
