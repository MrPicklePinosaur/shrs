//! Collection of utility functions for building a prompt

use std::{path::PathBuf, process::Command};

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
    let cur_dir = std::env::current_dir().unwrap();
    let home_dir = dirs::home_dir().unwrap();

    if cur_dir == home_dir {
        // home directory case
        String::from("~")
    } else if cur_dir == PathBuf::from("/") {
        // root directory case
        String::from("/")
    } else {
        cur_dir
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
    }
}

// TODO this is very linux specific, could use crate that abstracts
// TODO this function is disgusting
/// Get the username of the current user
pub fn username() -> anyhow::Result<String> {
    let username = Command::new("whoami").output()?.stdout;
    let encoded = std::str::from_utf8(&username)?
        .strip_suffix('\n')
        .unwrap()
        .to_string();
    Ok(encoded)
}

/// Get the hostname
pub fn hostname() -> anyhow::Result<String> {
    let username = Command::new("hostname").output()?.stdout;
    let encoded = std::str::from_utf8(&username)?
        .strip_suffix('\n')
        .unwrap()
        .to_string();
    Ok(encoded)
}

/// Get the current time
pub fn current_time() {
    todo!()
}
