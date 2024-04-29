//! Hooks that are defined by shrs_readline

use crossterm::event::KeyEvent;
use shrs_core_macros::Ctx;

use super::LineMode;
use crate::prelude::Ctx;

/// Runs whenever the current mode of the line changes
#[derive(Ctx)]
pub struct LineModeSwitchCtx {
    pub line_mode: LineMode,
}

#[derive(Ctx)]
pub struct ReadEventStartCtx;

#[derive(Ctx)]
pub struct PreRenderCtx {}
#[derive(Ctx)]
pub struct PostRenderCtx {}

#[derive(Ctx)]
pub struct ReadEventEndCtx;

#[derive(Ctx)]
pub struct OnKeyCtx {
    key: KeyEvent,
}
