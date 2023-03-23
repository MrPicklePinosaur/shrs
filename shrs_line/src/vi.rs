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
    MoveFindChar(char),
    MoveNextWord,
}

pub trait ViCursorBuffer {
    fn execute_vi(&mut self, action: ViAction) -> Result<()>;
}

impl ViCursorBuffer for CursorBuffer {
    fn execute_vi(&mut self, action: ViAction) -> Result<()> {
        match action {
            ViAction::MoveLeft => self.move_cursor(Location::Before()),
            ViAction::MoveRight => self.move_cursor(Location::After()),
            ViAction::MoveStart => self.move_cursor(Location::Front()),
            ViAction::MoveEnd => self.move_cursor(Location::Back(self)),
            // TODO should error be returned here instead? (currently results in NO-OP)
            ViAction::MoveFindChar(c) => {
                self.move_cursor(Location::FindChar(self, c).unwrap_or_default())
            },
            ViAction::MoveNextWord => {
                if let Some(cur_char) = self.char_at(Location::Cursor()) {
                    if !cur_char.is_whitespace() {
                        // if not whitespace first seek to whitespace character
                        self.move_cursor(
                            Location::Find(self, |ch| ch.is_whitespace()).unwrap_or_default(),
                        )?;
                    }
                    let loc = Location::Find(self, |ch| !ch.is_whitespace()).unwrap_or_default();
                    self.move_cursor(loc)
                } else {
                    Ok(())
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ViAction, ViCursorBuffer};
    use crate::cursor_buffer::{CursorBuffer, Location, Result};

    #[test]
    fn word_motions() -> Result<()> {
        let mut cb = CursorBuffer::from_str("hello world goodbye world");

        cb.execute_vi(ViAction::MoveNextWord)?;
        assert_eq!(cb.cursor(), 6);

        cb.execute_vi(ViAction::MoveNextWord)?;
        assert_eq!(cb.cursor(), 12);

        Ok(())
    }
}
