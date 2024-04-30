//! Hooks that are defined by shrs_readline

use crossterm::event::KeyEvent;

use super::LineMode;
use crate::prelude::HookCtx;

/// Runs whenever the current mode of the line changes
#[derive(HookCtx)]
pub struct LineModeSwitchCtx {
    pub line_mode: LineMode,
}

#[derive(HookCtx)]
pub struct ReadEventStartCtx;

#[derive(HookCtx)]
pub struct PreRenderCtx {}

#[derive(HookCtx)]
pub struct PostRenderCtx {}

#[derive(HookCtx)]
pub struct ReadEventEndCtx;

#[derive(HookCtx)]
pub struct OnKeyCtx {
    key: KeyEvent,
}
