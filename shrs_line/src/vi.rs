use crate::cursor_buffer::{CursorBuffer, Location, Result};

/// All actions that can be performed on a text buffer
///
/// Higher level abstraction over CursorBuffer operations
pub enum ViAction {
    MoveLeft,
    MoveRight,
    MoveStart,
    MoveEnd,
    /// Move cursor to point to the next character found
    MoveForwardFindChar(char),
}

impl ViAction {
    pub fn execute(&self, cb: &mut CursorBuffer) -> Result<()> {
        match self {
            ViAction::MoveLeft => cb.move_cursor(Location::Before()),
            ViAction::MoveRight => cb.move_cursor(Location::After()),
            ViAction::MoveStart => cb.move_cursor(Location::Front()),
            ViAction::MoveEnd => cb.move_cursor(Location::Back(cb)),
            // TODO should error be returned here instead? (currently results in NO-OP)
            ViAction::MoveForwardFindChar(c) => {
                cb.move_cursor(Location::FindChar(cb, *c).unwrap_or_default())
            },
        }
    }
}
