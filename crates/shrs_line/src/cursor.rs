//! Configuration for cursor

use crossterm::cursor::SetCursorStyle;

#[derive(Clone)]
pub struct CursorStyle {
    /// The cursor style used by crossterm to draw the cursor
    pub style: SetCursorStyle,
}

impl Default for CursorStyle {
    fn default() -> Self {
        Self {
            style: SetCursorStyle::DefaultUserShape,
        }
    }
}
