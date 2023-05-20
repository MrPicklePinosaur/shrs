//! Utilities for git repositories

use std::{path::PathBuf, process::Command, str};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("git command failed: {0}")]
    GitError(String),
    #[error("not in git repository")]
    NotGitRepo,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Get the top level directory of the git repository
pub fn root_dir() -> Result<PathBuf> {
    let res = Command::new("git")
        .args(vec!["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| Error::GitError(e.to_string()))?;

    Ok(PathBuf::from(str::from_utf8(&res.stdout).unwrap()))
}

/// Get name of current branch
pub fn branch() -> Result<String> {
    let res = Command::new("git")
        .args(vec!["branch", "--show-current"])
        .output()
        .map_err(|e| Error::GitError(e.to_string()))?;

    if !res.status.success() {
        return Err(Error::NotGitRepo);
    }

    Ok(str::from_utf8(&res.stdout).unwrap().trim().to_string())
}
