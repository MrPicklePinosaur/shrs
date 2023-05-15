/// Vi mode for readline
use shrs_utils::cursor_buffer::{CursorBuffer, Location, Result};
use shrs_vi::{Action, Motion};

use crate::LineMode;

/// Extension trait to [CursorBuffer] that enables the execution of vi motions
pub trait ViCursorBuffer {
    fn motion_to_loc(&self, motion: Motion) -> Result<Location>;
    fn execute_vi(&mut self, action: Action) -> Result<LineMode>;
}

impl ViCursorBuffer for CursorBuffer {
    fn motion_to_loc(&self, motion: Motion) -> Result<Location> {
        match motion {
            Motion::Find(c) => {
                // if current char is character we are looking for go to next one
                let offset = match self.char_at(Location::Cursor()) {
                    Some(cur_char) if cur_char == c => Location::After(),
                    _ => Location::Cursor(),
                };
                Ok(Location::FindChar(self, offset, c).unwrap_or_default())
            },
            Motion::Left => Ok(Location::Before()),
            Motion::Right => Ok(Location::After()),
            Motion::Start => Ok(Location::Front()),
            Motion::End => Ok(Location::Back(self)),
            Motion::Word => {
                // check if at end of line
                let cur_char = if let Some(ch) = self.char_at(Location::Cursor()) {
                    ch
                } else {
                    return Ok(Location::Cursor());
                };

                let start = if !cur_char.is_whitespace() {
                    // if not whitespace first seek to whitespace character
                    Location::Find(self, Location::Cursor(), |ch| ch.is_whitespace())
                        .unwrap_or(Location::Back(self))
                } else {
                    Location::Cursor()
                };
                Ok(Location::Find(self, start, |ch| !ch.is_whitespace())
                    .unwrap_or(Location::Back(self)))
            },
            Motion::WordPunc => {
                let punc = "!-~*|\".?[]{}()";
                //check if at end of line
                let cur_char = if let Some(ch) = self.char_at(Location::Cursor()) {
                    ch
                } else {
                    return Ok(Location::Cursor());
                };
                let start = if cur_char.is_whitespace() {
                    Location::Cursor()
                } else if punc.contains(cur_char) {
                    //start at non punc
                    Location::Find(self, Location::Cursor(), |ch| !punc.contains(ch))
                        .unwrap_or(Location::Back(self))
                } else {
                    //if letter char
                    Location::Find(self, Location::Cursor(), |ch| {
                        ch.is_whitespace() || punc.contains(ch)
                    })
                    .unwrap_or(Location::Back(self))
                };

                //jump to next word
                Ok(Location::Find(self, start, |ch| !ch.is_whitespace())
                    .unwrap_or(Location::Back(self)))
            },
            Motion::BackWord => {
                // TODO logic is getting comlpicatied, need more predicates to test location of
                // cursor (is cursor on first char of word, last char of word etc)

                // Move to the beginning of previous word
                let offset = match self.char_at(Location::Cursor()) {
                    Some(cur_char) if cur_char.is_whitespace() => {
                        Location::FindBack(self, Location::Cursor(), |ch| !ch.is_whitespace())
                            .unwrap_or(Location::Front())
                    },
                    _ => match self.char_at(Location::Before()) {
                        Some(before) if before.is_whitespace() => {
                            Location::FindBack(self, Location::Before(), |ch| !ch.is_whitespace())
                                .unwrap_or(Location::Front())
                        },
                        _ => Location::Cursor(),
                    },
                };

                let ret = match Location::FindBack(self, offset, |ch| ch.is_whitespace()) {
                    Some(back) => back,
                    None => return Ok(Location::Front()),
                };
                Ok(ret + Location::After())
            },
            _ => Ok(Location::Cursor()),
        }
    }

    fn execute_vi(&mut self, action: Action) -> Result<LineMode> {
        match action {
            Action::Insert => Ok(LineMode::Insert),
            Action::Move(motion) => match motion {
                Motion::Left
                | Motion::Right
                | Motion::Start
                | Motion::End
                | Motion::Word
                | Motion::WordPunc
                | Motion::BackWord
                | Motion::Find(_) => {
                    self.move_cursor(self.motion_to_loc(motion)?)?;
                    Ok(LineMode::Normal)
                },
                _ => Ok(LineMode::Normal),
            },
            Action::Delete(motion) => match motion {
                Motion::All => {
                    self.clear();
                    Ok(LineMode::Normal)
                },
                Motion::Left
                | Motion::Right
                | Motion::Start
                | Motion::End
                | Motion::Word
                | Motion::WordPunc
                | Motion::BackWord
                | Motion::Find(_) => {
                    self.delete(Location::Cursor(), self.motion_to_loc(motion)?)?;
                    Ok(LineMode::Normal)
                },
                _ => Ok(LineMode::Normal),
            },
            //executed left to right
            Action::Chain(action1, action2) => {
                self.execute_vi(*action1)?;
                self.execute_vi(*action2)
            },

            _ => Ok(LineMode::Normal),
        }
    }
}

#[cfg(test)]
mod test {
    use shrs_utils::cursor_buffer::{CursorBuffer, Result};
    use shrs_vi::{Action, Motion};

    use super::ViCursorBuffer;

    #[test]
    fn move_next_word() -> Result<()> {
        let mut cb = CursorBuffer::from_str("hello world goodbye world");

        assert_eq!(cb.cursor(), 0);

        cb.execute_vi(Action::Move(Motion::Word))?;
        assert_eq!(cb.cursor(), 6);

        cb.execute_vi(Action::Move(Motion::Word))?;
        assert_eq!(cb.cursor(), 12);

        Ok(())
    }

    #[test]
    fn move_back_word() -> Result<()> {
        let mut cb = CursorBuffer::from_str("hello world goodbye world");
        cb.execute_vi(Action::Move(Motion::End))?;
        cb.execute_vi(Action::Move(Motion::Left))?;
        assert_eq!(cb.cursor(), 24);

        cb.execute_vi(Action::Move(Motion::BackWord))?;
        assert_eq!(cb.cursor(), 20);

        cb.execute_vi(Action::Move(Motion::BackWord))?;
        assert_eq!(cb.cursor(), 12);

        Ok(())
    }
}
