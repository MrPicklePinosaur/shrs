//! Shell prompt

use shrs_utils::{styled_buf, styled_buf::StyledBuf};

use crate::prelude::States;

/// Implement this trait to create your own prompt
pub trait Prompt {
    fn prompt_left(&self, ctx: &States) -> StyledBuf;
    fn prompt_right(&self, ctx: &States) -> StyledBuf;
}

/// Default implementation for [Prompt]
#[derive(Default)]
pub struct DefaultPrompt {}

impl Prompt for DefaultPrompt {
    // TODO i still don't like passing all this context down
    fn prompt_left(&self, ctx: &States) -> StyledBuf {
        styled_buf!("> ")
    }

    fn prompt_right(&self, ctx: &States) -> StyledBuf {
        styled_buf!()
    }
}
