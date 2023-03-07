//! Collection of utility functions for building a prompt

use std::{borrow::Cow, ffi::OsString, process::Command};

use anyhow::anyhow;

/// Get the full working directory
pub fn full_pwd() -> String {
    std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

/// Get the top level working directory
pub fn top_pwd() -> String {
    std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap()
}

// TODO this is very linux specific, could use crate that abstracts
// TODO this function is disgusting
pub fn username() -> anyhow::Result<String> {
    let username = Command::new("whoami").output()?.stdout;
    let encoded = std::str::from_utf8(&username)?
        .strip_suffix("\n")
        .unwrap()
        .to_string();
    Ok(encoded)
}

pub fn hostname() -> anyhow::Result<String> {
    let username = Command::new("hostname").output()?.stdout;
    let encoded = std::str::from_utf8(&username)?
        .strip_suffix("\n")
        .unwrap()
        .to_string();
    Ok(encoded)
}
