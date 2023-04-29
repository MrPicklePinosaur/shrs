//! Shell prompt

use crossterm::style::{ContentStyle, StyledContent};

use crate::painter::StyledBuf;

pub trait Prompt {
    fn prompt_left(&self) -> StyledBuf;
    fn prompt_right(&self) -> StyledBuf;
}

/// Default implementation for [Prompt]
pub struct DefaultPrompt {}

impl DefaultPrompt {
    pub fn new() -> Self {
        DefaultPrompt {}
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
}
