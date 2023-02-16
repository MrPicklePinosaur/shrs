//! Collection of utility functions for building a prompt

use std::{ffi::OsString, process::Command};

use anyhow::anyhow;

/// Get the full working directory
pub fn full_pwd() -> OsString {
    std::env::current_dir()
        .map(|x| x.into_os_string())
        .unwrap_or(OsString::new())
}

/// Get the top level working directory
pub fn top_pwd() -> OsString {
    std::env::current_dir()
        .ok()
        .and_then(|x| x.file_name().map(|x| x.to_os_string()))
        .unwrap_or(OsString::new())
}

// TODO this is very linux specific, could use crate that abstracts
// TODO this function is disgusting
pub fn username() -> anyhow::Result<String> {
    let username = Command::new("whoami").output()?.stdout;
    let encoded = std::str::from_utf8(&username)?.to_string();
    Ok(encoded)
}

pub fn hostname() -> anyhow::Result<String> {
    let username = Command::new("hostname").output()?.stdout;
    let encoded = std::str::from_utf8(&username)?.to_string();
    Ok(encoded)
}
