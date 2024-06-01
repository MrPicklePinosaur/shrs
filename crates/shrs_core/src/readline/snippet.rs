//! Programmable text substitutions
//!
//! Snippets are substitutions that apply in the line when a trigger key is pressed. When a given
//! snippet is triggered it will expand based on a behavior to another string.
//! ```
//! # use shrs_core::prelude::*;
//! // Define all your snippets
//! let mut snippets = Snippets::new(ExpandSnippet::OnSpace);
//! snippets.add(
//!     "gc".to_string(),
//!     SnippetInfo::new("git commit -m \"", InsertPosition::Command),
//! );
//! snippets.add(
//!     "ga".to_string(),
//!     SnippetInfo::new("git add .", InsertPosition::Command),
//! );
//!
//! // Register your snippets with the shell
//! let shell = ShellBuilder::default().with_snippets(snippets);
//! ```

use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

/// Controls when snippet should be applied
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum ExpandSnippet {
    // Expand the snippet when tab is pressed
    OnTab,
    // Expand the snippet when space is pressed
    OnSpace,
    // Don't expand the snippet
    #[default]
    Never,
    // Expand the snippet when a specific key is pressed
    OnKey(KeyEvent),
}

/// Controls where a snippet is allowed to be expanded
#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum InsertPosition {
    /// Only expand the snippet if it is the first argument
    #[default]
    Command,
    /// Expand the snippet anywhere
    Anywhere,
}

/// Configure what a snippet expands to and how it is expanded
pub struct SnippetInfo {
    /// Value to be inserted
    pub value: String,
    /// Where the snippet needs to be, to be expanded
    pub position: InsertPosition,
}

impl SnippetInfo {
    pub fn new<S: ToString>(value: S, position: InsertPosition) -> Self {
        Self {
            value: value.to_string(),
            position,
        }
    }
}

/// Shell state to hold registered snippets
#[derive(Default)]
pub struct Snippets {
    snippets: HashMap<String, SnippetInfo>,
    expand_snippet: ExpandSnippet,
    enabled: bool,
}

impl Snippets {
    pub fn new(expand_snippet: ExpandSnippet) -> Self {
        Self {
            expand_snippet,
            snippets: HashMap::new(),
            enabled: true,
        }
    }

    /// Register a new snippet
    ///
    /// If a snippet of the existing name has already been registered, it will be overwritten
    pub fn add(&mut self, trigger: String, info: SnippetInfo) {
        self.snippets.insert(trigger, info);
    }

    /// Returns whether the event was matched or not.
    pub fn should_expand(&self, event: &Event) -> bool {
        if !self.enabled {
            return false;
        }
        match self.expand_snippet {
            ExpandSnippet::OnSpace => {
                *event == Event::Key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE))
            },
            ExpandSnippet::OnTab => {
                *event == Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE))
            },

            ExpandSnippet::Never => false,
            ExpandSnippet::OnKey(k) => *event == Event::Key(k),
        }
    }

    /// Fetch what a snippet expands to given a trigger string
    pub fn get(&self, trigger: &String) -> Option<&SnippetInfo> {
        self.snippets.get(trigger)
    }

    /// Allow snippets to be expanded
    pub fn enable(&mut self) {
        self.enabled = true
    }

    /// Don't allow snippets to be expanded
    pub fn disable(&mut self) {
        self.enabled = false
    }

    /// Check if snippets are allowed to be expanded
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
