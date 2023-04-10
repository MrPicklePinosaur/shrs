use shrs_vi::{Action, Command, Motion};

/// Vi mode for readline
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
    MoveBackWord,
}

/// Extension trait to [CursorBuffer] that enables the execution of vi motions
pub trait ViCursorBuffer {
    fn execute_vi(&mut self, action: Action) -> Result<()>;
}

impl ViCursorBuffer for CursorBuffer {
    fn execute_vi(&mut self, action: Action) -> Result<()> {
        match action {
            Action::Move(motion) => {
                match motion {
                    Motion::Left => self.move_cursor(Location::Before()),
                    Motion::Right => self.move_cursor(Location::After()),
                    Motion::Start => self.move_cursor(Location::Front()),
                    Motion::End => self.move_cursor(Location::Back(self)),
                    Motion::Word => {
                        // TODO if at end, just seek to very end of line
                        if let Some(cur_char) = self.char_at(Location::Cursor()) {
                            if !cur_char.is_whitespace() {
                                // if not whitespace first seek to whitespace character
                                self.move_cursor(
                                    Location::Find(self, |ch| ch.is_whitespace())
                                        .unwrap_or_default(),
                                )?;
                            }
                            self.move_cursor(
                                Location::Find(self, |ch| !ch.is_whitespace()).unwrap_or_default(),
                            )
                        } else {
                            Ok(())
                        }
                    },
                }
            },
            _ => Ok(()), /*
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
                                 self.move_cursor(
                                     Location::Find(self, |ch| !ch.is_whitespace()).unwrap_or_default(),
                                 )
                             } else {
                                 Ok(())
                             }
                         },
                         ViAction::MoveBackWord => {
                             // TODO logic is getting comlpicatied, need more predicates to test location of
                             // cursor (is cursor on first char of word, last char of word etc)

                             // Move to the beginning of previous word
                             if let Some(cur_char) = self.char_at(Location::Cursor()) {
                                 if cur_char.is_whitespace() {
                                     // if whitespace seek back to word first
                                     self.move_cursor(
                                         Location::FindBack(self, |ch| !ch.is_whitespace()).unwrap_or_default(),
                                     )?;
                                 } else {
                                     // and if is first letter of word
                                     if let Some(before) = self.char_at(Location::Before()) {
                                         if before.is_whitespace() {
                                             self.move_cursor(Location::Before())?;
                                             // if whitespace seek back to word first
                                             self.move_cursor(
                                                 Location::FindBack(self, |ch| !ch.is_whitespace())
                                                     .unwrap_or_default(),
                                             )?;
                                         }
                                     }
                                 }

                                 self.move_cursor(
                                     Location::FindBack(self, |ch| ch.is_whitespace()).unwrap_or_default()
                                         + Location::After(),
                                 )
                             } else {
                                 Ok(())
                             }
                         },
                         */
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ViAction, ViCursorBuffer};
    use crate::cursor_buffer::{CursorBuffer, Location, Result};

    #[test]
    fn move_next_word() -> Result<()> {
        let mut cb = CursorBuffer::from_str("hello world goodbye world");

        cb.execute_vi(ViAction::MoveNextWord)?;
        assert_eq!(cb.cursor(), 6);

        cb.execute_vi(ViAction::MoveNextWord)?;
        assert_eq!(cb.cursor(), 12);

        Ok(())
    }

    #[test]
    fn move_back_word() -> Result<()> {
        let mut cb = CursorBuffer::from_str("hello world goodbye world");
        cb.execute_vi(ViAction::MoveEnd)?;
        cb.execute_vi(ViAction::MoveLeft)?;
        assert_eq!(cb.cursor(), 24);

        cb.execute_vi(ViAction::MoveBackWord)?;
        assert_eq!(cb.cursor(), 20);

        cb.execute_vi(ViAction::MoveBackWord)?;
        assert_eq!(cb.cursor(), 12);

        Ok(())
    }
}
