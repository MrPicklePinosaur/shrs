//! Utilities for git repositories

use std::{path::PathBuf, process::Command, str};

use shrs::anyhow;
use thiserror::Error;

use crate::query::{Query, QueryBuilder, QueryBuilderError, QueryResult};

#[derive(Debug)]
pub struct Git {
    pub branch: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("git command failed: {0}")]
    GitError(String),
    #[error("not in git repository")]
    NotGitRepo,
}

/// Get the top level directory of the git repository
pub fn root_dir() -> anyhow::Result<PathBuf> {
    let res = Command::new("git")
        .args(vec!["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| Error::GitError(e.to_string()))?;

    Ok(PathBuf::from(str::from_utf8(&res.stdout).unwrap()))
}

/// Get name of current branch
pub fn branch() -> anyhow::Result<String> {
    let res = Command::new("git")
        .args(vec!["branch", "--show-current"])
        .output()
        .map_err(|e| Error::GitError(e.to_string()))?;

    if !res.status.success() {
        return Err(anyhow::anyhow!(Error::NotGitRepo));
    }

    Ok(str::from_utf8(&res.stdout).unwrap().trim().to_string())
}
pub fn commits_behind_remote() -> anyhow::Result<u32> {
    let res = Command::new("git")
        .args(vec![
            "rev-list",
            "--right-only",
            "--count",
            "HEAD...@{upstream}",
        ])
        .output()
        .map_err(|e| Error::GitError(e.to_string()))?;
    Ok(str::from_utf8(&res.stdout).unwrap().trim().parse::<u32>()?)
}
pub fn commits_ahead_remote() -> anyhow::Result<u32> {
    let res = Command::new("git")
        .args(vec![
            "rev-list",
            "--left-only",
            "--count",
            "HEAD...@{upstream}",
        ])
        .output()
        .map_err(|e| Error::GitError(e.to_string()))?;
    Ok(str::from_utf8(&res.stdout).unwrap().trim().parse::<u32>()?)
}

pub fn latest_commit_summary() -> anyhow::Result<String> {
    let res = Command::new("git")
        .args(vec!["show", "--pretty=%s", "--no-patch", "HEAD"])
        .output()
        .map_err(|e| Error::GitError(e.to_string()))?;
    Ok(str::from_utf8(&res.stdout).unwrap().trim().to_string())
}

fn metadata_fn(query_res: &mut QueryResult) -> anyhow::Result<()> {
    query_res.add_metadata(Git { branch: branch()? });

    Ok(())
}

pub fn module() -> Result<Query, QueryBuilderError> {
    QueryBuilder::default()
        .metadata_fn(Box::new(metadata_fn))
        .build()
}
