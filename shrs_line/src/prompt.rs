//! Shell prompt

use crossterm::style::{ContentStyle, StyledContent};

use crate::painter::StyledBuf;

pub trait Prompt {
    fn prompt_left(&self) -> StyledBuf;
    fn prompt_right(&self) -> StyledBuf;
    fn set_left(&mut self, field: String);
    fn set_right(&mut self, field: String);
}

/// Default implementation for [Prompt]
pub struct DefaultPrompt {
    left_field: String,
    right_field: String,
}

impl DefaultPrompt {
    pub fn new() -> Self {
        DefaultPrompt {
            left_field: "".into(),
            right_field: "".into(),
        }
    }
}

impl Prompt for DefaultPrompt {
    fn prompt_left(&self) -> StyledBuf {
        StyledBuf::from_iter(vec![StyledContent::new(
            ContentStyle::new(),
            "> ".to_string(),
        )])
    }

    fn prompt_right(&self) -> StyledBuf {
        StyledBuf::from_iter(vec![])
    }

    fn set_left(&mut self, field: String) {
        self.left_field = field.into();
    }

    fn set_right(&mut self, field: String) {
        self.right_field = field.into();
    }
}
