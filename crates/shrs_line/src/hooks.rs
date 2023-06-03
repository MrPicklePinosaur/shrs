//! Hooks that are defined by shrs_readline

use crate::line::LineMode;

/// Runs whenever the current mode of the line changes
#[derive(Clone)]
pub struct LineModeSwitchCtx {
    pub line_mode: LineMode,
}
