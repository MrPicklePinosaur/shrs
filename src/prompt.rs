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

// prompt for reedline

pub struct CustomPrompt {
    pub prompt_indicator: String,
    pub prompt_left: String,
    pub prompt_right: String,
    pub multiline_indicator: String,
}

impl Default for CustomPrompt {
    fn default() -> Self {
        CustomPrompt {
            prompt_indicator: "> ".into(),
            prompt_left: "sh".into(),
            prompt_right: "shrs".into(),
            multiline_indicator: "| ".into(),
        }
    }
}

impl Prompt for CustomPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::Owned(self.prompt_left.clone())
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Owned(self.prompt_right.clone())
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<str> {
        Cow::Owned(self.prompt_indicator.clone())
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Borrowed(&self.multiline_indicator)
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
}
