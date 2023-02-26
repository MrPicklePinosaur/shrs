//! Collection of utility functions for building a prompt

use std::{borrow::Cow, ffi::OsString, fmt::Display, process::Command};

use anyhow::anyhow;
use reedline::{
    Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus, Reedline, Signal,
};

// pub trait Section {
// }

// pub struct ShowSection<T>(pub T);
// impl<T> std::fmt::Display for ShowSection<T>
// where T: Section
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 	write!(f, "{}", self.0.render())
//     }
// }

pub enum WorkDir {
    Full,
    Top,
}

impl Display for WorkDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            WorkDir::Full => std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
            WorkDir::Top => std::env::current_dir()
                .unwrap()
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        };
        write!(f, "{}", str)
    }
}

// TODO could technically cache username and hostname as they are unlikely to change
pub struct Username;

impl Display for Username {
    // TODO this is very linux specific, could use crate that abstracts
    // TODO this function is disgusting
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let username = Command::new("whoami").output().unwrap().stdout;
        let str = std::str::from_utf8(&username)
            .unwrap()
            .strip_suffix("\n")
            .unwrap()
            .to_string();
        write!(f, "{}", str)
    }
}

pub struct Hostname;

impl Display for Hostname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let username = Command::new("hostname").output().unwrap().stdout;
        let str = std::str::from_utf8(&username)
            .unwrap()
            .strip_suffix("\n")
            .unwrap()
            .to_string();
        write!(f, "{}", str)
    }
}

pub struct Rust;

impl Display for Rust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO grab info from Cargo.toml

        // look for Cargo.toml
        for path in std::fs::read_dir("./").unwrap() {
            if path.unwrap().file_name() == "Cargo.toml" {
                return write!(f, "rust");
            }
        }
        write!(f, "")
    }
}

// prompt for reedline

pub trait CustomPrompt: Send {
    fn prompt_indicator(&self) -> String;
    fn prompt_left(&self) -> String;
    fn prompt_right(&self) -> String;
    fn multiline_indicator(&self) -> String;
}

pub struct SimplePrompt;

impl CustomPrompt for SimplePrompt {
    fn prompt_indicator(&self) -> String {
        "> ".into()
    }
    fn prompt_left(&self) -> String {
        let username = Username;
        let hostname = Hostname;
        let workdir = WorkDir::Top;
        format!("{username}@{hostname} {workdir} > ")
    }
    fn prompt_right(&self) -> String {
        "".into()
    }
    fn multiline_indicator(&self) -> String {
        "| ".into()
    }
}

pub struct PromptWrapper<T>(pub T);
impl<T> Prompt for PromptWrapper<T>
where
    T: CustomPrompt,
{
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Owned(self.0.prompt_left())
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Owned(self.0.prompt_right())
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<str> {
        Cow::Owned(self.0.prompt_indicator())
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Owned(self.0.multiline_indicator())
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };

        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::prompt::*;

    #[test]
    fn simple_prompt() {
        let username = Username;
        let hostname = Hostname;
        let workdir = WorkDir::Top;
        println!("{username}@{hostname} {workdir} > ");
    }

    #[test]
    fn rust_prompt() {
        let rust = Rust;
        println!("{rust} > ");
    }
}
