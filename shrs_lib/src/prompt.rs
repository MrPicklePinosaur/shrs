//! Collection of utility functions for building a prompt

use std::{borrow::Cow, ffi::OsString, process::Command};

use anyhow::anyhow;
use reedline::{
    Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus, Reedline, Signal,
};

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
