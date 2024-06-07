//! Utilities for git repositories

use std::env::current_dir;

use anyhow::Result;
use git2::Repository;
use shrs::prelude::*;
use thiserror::Error;
pub struct Git {
    repository: Option<Repository>,
}
#[derive(Error, Debug)]
pub enum GitError {
    #[error("git command failed: {0}")]
    GitError(String),
    #[error("not in git repository")]
    NotGitRepo,
}
impl Git {
    pub fn new() -> Self {
        Self { repository: None }
    }
    pub fn discover(&mut self) {
        self.repository = Repository::discover(current_dir().unwrap()).ok();
    }
    pub fn stashes(&mut self) -> Result<usize> {
        let mut count = 0;

        if let Some(repo) = &mut self.repository {
            repo.stash_foreach(|i, s, o| {
                count += 1;
                true
            })?;
        } else {
            return Err(anyhow::anyhow!(GitError::NotGitRepo));
        }
        Ok(count)
    }
    pub fn is_repo(&self) -> bool {
        self.repository.is_some()
    }
    pub fn current_branch(&mut self) {}
    pub fn commits_ahead() {}
    pub fn commits_behind() {}
}
pub struct GitPlugin;
impl Plugin for GitPlugin {
    fn init(&self, config: &mut ShellConfig) -> anyhow::Result<()> {
        config.states.insert(Git::new());
        config.hooks.insert(
            |mut git: StateMut<Git>, sh: &Shell, _: &StartupCtx| -> Result<()> {
                git.discover();
                Ok(())
            },
        );
        config.hooks.insert(
            |mut git: StateMut<Git>, sh: &Shell, _: &ChangeDirCtx| -> Result<()> {
                git.discover();
                Ok(())
            },
        );

        Ok(())
    }
}
