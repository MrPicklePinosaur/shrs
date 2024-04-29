//! Hooks that are defined by shrs_readline

use crossterm::event::KeyEvent;

use super::LineMode;
use crate::prelude::HookCtx;

/// Runs whenever the current mode of the line changes
#[derive(HookCtx, Clone)]
pub struct LineModeSwitchCtx {
    pub line_mode: LineMode,
}

#[derive(HookCtx, Clone)]
pub struct ReadEventStartCtx;

#[derive(HookCtx, Clone)]
pub struct PreRenderCtx {}

#[derive(HookCtx, Clone)]
pub struct PostRenderCtx {}

#[derive(HookCtx, Clone)]
pub struct ReadEventEndCtx;

#[derive(HookCtx, Clone)]
pub struct OnKeyCtx {
    key: KeyEvent,
}
