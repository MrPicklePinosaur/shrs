//! Shell prompt

use shrs_utils::{styled_buf, styled_buf::StyledBuf};

use super::LineStateBundle;

/// Implement this trait to create your own prompt
pub trait Prompt {
    fn prompt_left(&self, line_ctx: &LineStateBundle) -> StyledBuf;
    fn prompt_right(&self, line_ctx: &LineStateBundle) -> StyledBuf;
}

/// Default implementation for [Prompt]
#[derive(Default)]
pub struct DefaultPrompt {}

impl Prompt for DefaultPrompt {
    // TODO i still don't like passing all this context down
    fn prompt_left(&self, _line_ctx: &LineStateBundle) -> StyledBuf {
        styled_buf!("> ")
    }

    fn prompt_right(&self, _line_ctx: &LineStateBundle) -> StyledBuf {
        styled_buf!()
    }
}
