//! Configuration for cursor

use crossterm::cursor::SetCursorStyle;

#[derive(Clone)]
pub struct CursorStyle {
    pub style: SetCursorStyle,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self {
            style: SetCursorStyle::DefaultUserShape,
        }
    }
}
