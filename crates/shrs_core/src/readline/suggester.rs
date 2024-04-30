use super::line::LineContents;
use crate::prelude::{Shell, States};

pub trait Suggester {
    fn suggest(&self, sh: &Shell, states: &States) -> Option<String>;
}
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
