//! Shell prompt

use crossterm::style::{ContentStyle, StyledContent};
use shrs_core::{Context, Runtime, Shell};

use crate::{line::LineCtx, painter::StyledBuf};

pub trait Prompt {
    fn prompt_left(&self, line_ctx: &mut LineCtx) -> StyledBuf;
    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf;
}

/// Default implementation for [Prompt]
pub struct DefaultPrompt {}

impl DefaultPrompt {
    pub fn new() -> Self {
        DefaultPrompt {}
    }
}

impl Prompt for DefaultPrompt {
    // TODO i still don't like passing all this context down
    fn prompt_left(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        StyledBuf::from_iter(vec![StyledContent::new(
            ContentStyle::new(),
            "> ".to_string(),
        )])
    }

    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        StyledBuf::from_iter(vec![])
    }
}
