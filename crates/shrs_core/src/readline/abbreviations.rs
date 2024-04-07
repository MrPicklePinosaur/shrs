use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use super::line::LineState;

///Controls when abbreviations should be applied
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum ExpandAbbreviation {
    Always,
    #[default]
    Never,
    OnKey(KeyEvent),
}

/// Abbreviations are applied
#[derive(Default)]
pub struct Abbreviations {
    abbrs: HashMap<String, String>,
    expand_abbreviation: ExpandAbbreviation,
}
impl Abbreviations {
    pub fn new<T: ToString>(abbrs: HashMap<T, T>, expand_abbreviation: ExpandAbbreviation) -> Self {
        let abbrs = abbrs
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        Self {
            abbrs,
            expand_abbreviation,
        }
    }
    /// Returns whether the event was matched or not.
    /// Always is mapped to when the user presses space
    pub fn should_expand(&self, event: &Event) -> bool {
        match self.expand_abbreviation {
            ExpandAbbreviation::Always => {
                *event == Event::Key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE))
            },
            ExpandAbbreviation::Never => false,
            ExpandAbbreviation::OnKey(k) => *event == Event::Key(k),
        }
    }
    pub fn get(&self, name: &String) -> Option<&String> {
        self.abbrs.get(name)
    }
}
