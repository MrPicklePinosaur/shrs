//! Friendly wrapper around Rope data structure that includes a cursor as well as relative and
//! absolute indexing
use std::ops::RangeBounds;

use ropey::{Rope, RopeSlice};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid relative offset {0}")]
    InvalidRelativeLocation(isize),
    #[error("Invalid absolute index {0}")]
    InvalidAbsoluteLocation(usize),
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Location {
    /// Absolute location
    Abs(usize),
    /// Relative location from cursor
    Rel(isize),
}

#[allow(non_snake_case)]
impl Location {
    /// Location at the cursor, alias of `Location::Rel(0)`
    pub fn Cursor() -> Location {
        Location::Rel(0)
    }

    /// Location just before cursor, alias of `Location::Rel(-1)`
    pub fn Before() -> Location {
        Location::Rel(-1)
    }

    /// Location just after cursor, alias of `Location::Rel(1)`
    pub fn After() -> Location {
        Location::Rel(1)
    }

    /// Location at beginning of buffer, alias of `Location::Abs(0)`
    pub fn Front() -> Location {
        Location::Abs(0)
    }

    pub fn Back() -> Location {
        todo!()
    }
}

/// Friendly wrapper around Rope data structure
pub struct CursorBuffer {
    data: Rope,
    /// Cursor is an absolute index into the data buffer
    ///
    /// It always points **before** the character. A cursor value of 0 would pointer before the
    /// first character and a cursor value of `data.len_chars()` would point after the last character. This
    /// also means that the range of valid cursor values is `0..=data.len_chars()`
    ///
    /// Invariant: cursor is always valid (never need to perform bounds checking on `cursor` itself)
    cursor: usize,
}

impl CursorBuffer {
    /// Construct an empty CursorBuffer
    pub fn new() -> Self {
        CursorBuffer {
            data: Rope::new(),
            cursor: 0,
        }
    }

    /// Move the cursor using a location selector
    pub fn move_cursor(&mut self, loc: Location) -> Result<()> {
        self.cursor = self.to_absolute(loc)?;
        Ok(())
    }

    /// Insert text and advance cursor
    pub fn insert(&mut self, loc: Location, text: &str) -> Result<()> {
        self.append(loc, text)?;
        self.move_cursor(Location::Rel(text.len() as isize))?;
        Ok(())
    }

    /// Insert text without advancing cursor
    pub fn append(&mut self, loc: Location, text: &str) -> Result<()> {
        self.data.insert(self.to_absolute(loc)?, text);
        Ok(())
    }

    /// Get a slice of the text
    pub fn slice<R>(&self, char_range: R) -> RopeSlice<'_>
    where
        R: RangeBounds<usize>,
    {
        self.data.slice(char_range)
    }

    /// Getter for the current index of the cursor
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Converts `Location` to an absolute index into the buffer. Performs bounds checking
    // TODO to absolute would be much nice semantically if it was a method on `Location`, however
    // we need access to `data.len_chars()` and `cursor` to perform the conversion
    fn to_absolute(&self, loc: Location) -> Result<usize> {
        match loc {
            Location::Abs(i) => {
                if self.bounds_check(i as isize) {
                    Ok(i)
                } else {
                    Err(Error::InvalidAbsoluteLocation(i))
                }
            },
            Location::Rel(offset) => {
                let abs = self.cursor as isize + offset;
                if self.bounds_check(abs) {
                    // we know this cast will succeed (TODO would be better to do this without cast)
                    Ok(abs as usize)
                } else {
                    Err(Error::InvalidRelativeLocation(offset))
                }
            },
        }
    }

    /// Predicate if an index is a valid cursor into the buffer
    fn bounds_check(&self, i: isize) -> bool {
        i >= 0 && i <= self.data.len_chars() as isize
    }
}

#[cfg(test)]
mod tests {
    use super::{CursorBuffer, Location, Result};

    #[test]
    fn cursor_buffer() -> Result<()> {
        let mut cb = CursorBuffer::new();

        cb.insert(Location::Cursor(), "hello world")?;
        assert_eq!(cb.slice(..), "hello world");
        assert_eq!(cb.cursor(), 11);

        Ok(())
    }
}
