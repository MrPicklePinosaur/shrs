//! Shell prompt

use crossterm::style::{ContentStyle, StyledContent};
use shrs_utils::{styled, styled_buf::StyledBuf};

use crate::line::LineCtx;

/// Implement this trait to create your own prompt
pub trait Prompt {
    fn prompt_left(&self, line_ctx: &mut LineCtx) -> StyledBuf;
    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf;
}

/// Default implementation for [Prompt]
#[derive(Default)]
pub struct DefaultPrompt {}

impl Prompt for DefaultPrompt {
    // TODO i still don't like passing all this context down
    fn prompt_left(&self, _line_ctx: &mut LineCtx) -> StyledBuf {
        styled!("> ")
    }

    fn prompt_right(&self, _line_ctx: &mut LineCtx) -> StyledBuf {
        styled!()
    }
}
