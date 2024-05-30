//! Inline suggestions
//!
//! Suggester is a handler that provides inline suggestions.
//!
//! The suggestion is styled using the `suggestion_style` field in `Theme`.
use super::line::LineContents;
use crate::prelude::{Shell, States};

pub trait Suggester {
    fn suggest(&self, sh: &Shell, states: &States) -> Option<String>;
}
/// Default inline `Suggester`
///
/// Uses History items and selects the most recent command that starts with
pub struct DefaultSuggester;
impl Suggester for DefaultSuggester {
    fn suggest(&self, sh: &Shell, states: &States) -> Option<String> {
        let res = states.get_mut::<LineContents>().get_full_command();
        if res.is_empty() {
            return None;
        }

        for s in sh.history.items(sh, states) {
            if s.starts_with(&res) {
                return Some(s.to_owned());
            }
        }
        None
    }
}
