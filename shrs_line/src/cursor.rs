use crossterm::cursor::SetCursorStyle;

pub trait Cursor {
    fn get_cursor(&self) -> SetCursorStyle;
}

pub struct DefaultCursor {
    style: SetCursorStyle,
}

impl Default for DefaultCursor {
    fn default() -> Self {
        DefaultCursor {
            style: SetCursorStyle::DefaultUserShape,
        }
    }
}

impl Cursor for DefaultCursor {
    fn get_cursor(&self) -> SetCursorStyle {
        self.style
    }
}
