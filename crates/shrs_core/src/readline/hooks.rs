//! Hooks that are defined by shrs_readline

use crossterm::event::KeyEvent;

use crate::prelude::{HookEvent, HookEventMarker, LineMode};

/// Runs whenever the current mode of the line changes
#[derive(HookEvent)]
pub struct LineModeSwitchEvent {
    pub line_mode: LineMode,
}

#[derive(HookEvent)]
pub struct ReadEventStartEvent;

#[derive(HookEvent)]
pub struct PreRenderEvent {}

#[derive(HookEvent)]
pub struct PostRenderEvent {}

#[derive(HookEvent)]
pub struct ReadEventEndEvent;

#[derive(HookEvent)]
pub struct OnKeyEvent {
    key: KeyEvent,
}
