//! Friendly wrapper around Rope data structure that includes a cursor as well as relative and
//! absolute indexing
use ropey::Rope;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid relative offset {0}")]
    InvalidRelativeLocation(i32),
    #[error("Invalid absolute index {0}")]
    InvalidAbsoluteLocation(usize),
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Location {
    /// Absolute location
    Abs(usize),
    /// Relative location from cursor
    Rel(i32),
    /// At cursor position
    Cursor,
}

pub struct CursorBuffer {
    data: Rope,
    /// Cursor is an absolute index into the data buffer
    ///
    /// It always points **before** the character. A cursor value of 0 would pointer before the
    /// first character and a cursor value of `data.len_chars()` would point after the last character. This
    /// also means that the range of valid cursor values is `0..=data.len_chars()`
    cursor: usize,
}

impl CursorBuffer {
    pub fn new() {}

    pub fn move_cursor(&mut self, loc: Location) -> Result<()> {
        self.cursor = self.to_absolute(loc)?;
        Ok(())
    }

    /// Converts `Location` to an absolute index into the buffer. Performs bounds checking
    // TODO to absolute would be much nice semantically if it was a method on `Location`, however
    // we need access to `data.len_chars()` and `cursor` to perform the conversion
    fn to_absolute(&self, loc: Location) -> Result<usize> {
        match loc {
            Location::Abs(i) => {
                if self.bounds_check(i as i32) {
                    Ok(i)
                } else {
                    Err(Error::InvalidAbsoluteLocation(i))
                }
            },
            Location::Rel(offset) => {
                let abs = self.cursor as i32 + offset;
                if self.bounds_check(abs) {
                    // we know this cast will succeed (TODO would be better to do this without cast)
                    Ok(abs as usize)
                } else {
                    Err(Error::InvalidRelativeLocation(offset))
                }
            },
            Location::Cursor => Ok(self.cursor),
        }
    }

    /// Predicate if an index is a valid cursor into the buffer
    fn bounds_check(&self, i: i32) -> bool {
        i >= 0 && i <= self.data.len_chars() as i32
    }
}
