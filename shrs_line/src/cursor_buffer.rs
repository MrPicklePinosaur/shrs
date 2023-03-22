//! Friendly wrapper around Rope data structure that includes a cursor as well as relative and
//! absolute indexing
use std::ops::RangeBounds;

use ropey::{Rope, RopeSlice};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Invalid relative offset {0}")]
    InvalidRelativeLocation(isize),
    #[error("Invalid absolute index {0}")]
    InvalidAbsoluteLocation(usize),
    #[error("Deleting past end of buffer")]
    DeletingTooMuch,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Location {
    /// Absolute location
    Abs(usize),
    /// Relative location from cursor
    Rel(isize),
}

impl Default for Location {
    fn default() -> Self {
        Location::Cursor()
    }
}

// NOTE: something to consider, if we create a `Location` object using a reference to `CursorBuffer`, the `Location` may become invalidated if we modify the `CursorBuffer`. Should find way to invalidate a `Location` whenever `CursorBuffer` is mutated
// TODO: implement Add and Sub for location?
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

    /// Location at end of buffer
    pub fn Back(cb: &CursorBuffer) -> Location {
        Location::Abs(cb.len())
    }

    /// Location of the next occurance of character
    pub fn FindChar(cb: &CursorBuffer, c: char) -> Option<Location> {
        let ind = cb.chars(Location::Cursor()).unwrap().position(|ch| ch == c);
        ind.map(|i| Location::Abs(i))
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

    /// Create new `CursorBuffer` from string an sets cursor location to beginning
    pub fn from_str(text: &str) -> Self {
        CursorBuffer {
            data: Rope::from_str(text),
            cursor: 0,
        }
    }

    /// Move the cursor using a location selector
    pub fn move_cursor(&mut self, loc: Location) -> Result<()> {
        self.cursor = self.to_absolute(loc)?;
        Ok(())
    }

    /// Move the cursor using a location selector, clamping the cursor if it were to move to
    /// invalid position
    pub fn move_cursor_clamp(&mut self, loc: Location) {
        todo!()
    }

    /// Insert text and advance cursor to after the text inserted
    pub fn cursor_insert(&mut self, loc: Location, text: &str) -> Result<()> {
        self.data.insert(self.to_absolute(loc)?, text);
        self.move_cursor(loc)?;
        self.move_cursor(Location::Rel(text.len() as isize))?;
        Ok(())
    }

    /// Insert text and offset cursor to point to same text
    pub fn insert(&mut self, loc: Location, text: &str) -> Result<()> {
        todo!()
    }

    /// Delete a length of text starting from location and move cursor to start of deleted text
    pub fn cursor_delete(&mut self, loc: Location, len: usize) -> Result<()> {
        let start = self.to_absolute(loc)?;
        if start + len > self.len() {
            return Err(Error::DeletingTooMuch);
        }
        self.data.remove(start..start + len);
        self.move_cursor(Location::Abs(start))?;
        Ok(())
    }

    /// Delete a length of text starting from location and offset the cursor accordingly such that
    /// it points to the same text
    ///
    /// In the case that cursor was pointing at deleted text, the behavior is the same as
    /// `cursor_delete`
    pub fn delete(&mut self, loc: Location, len: usize) -> Result<()> {
        todo!()
    }

    /// Delete a length of text ending at location
    // TODO handle panic
    pub fn delete_before(&mut self, loc: Location, len: usize) -> Result<()> {
        todo!()
    }

    /// Empties all text and resets cursor
    pub fn clear(&mut self) {
        self.data.remove(..);
        self.cursor = 0;
    }

    /// Get a slice of the text
    pub fn slice<R>(&self, char_range: R) -> RopeSlice<'_>
    where
        R: RangeBounds<usize>,
    {
        self.data.slice(char_range)
    }

    /// Create forward iterator from a location
    // TODO: maybe wrap `ropey::iter::Chars` in a newtype
    pub fn chars(&self, loc: Location) -> Result<ropey::iter::Chars<'_>> {
        Ok(self.data.chars_at(self.to_absolute(loc)?))
    }

    /// Getter for the current index of the cursor
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Get the length of the text in number of characters
    pub fn len(&self) -> usize {
        self.data.len_chars()
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
        i >= 0 && i <= self.len() as isize
    }
}

#[cfg(test)]
mod tests {
    use super::{CursorBuffer, Error, Location, Result};

    #[test]
    /// Basic insert and delete test
    fn basic_insert_delete() -> Result<()> {
        let mut cb = CursorBuffer::new();

        cb.cursor_insert(Location::Cursor(), "hello world")?;
        assert_eq!(cb.slice(..), "hello world");
        assert_eq!(cb.cursor(), 11);

        cb.cursor_delete(Location::Front(), 6)?;
        assert_eq!(cb.slice(..), "world");
        assert_eq!(cb.cursor(), 0);

        Ok(())
    }

    #[test]
    /// Test overdeleting buffer
    fn over_delete() -> Result<()> {
        let mut cb = CursorBuffer::from_str("hello");

        assert_eq!(
            cb.cursor_delete(Location::Cursor(), 200),
            Err(Error::DeletingTooMuch)
        );
        Ok(())
    }

    #[test]
    fn find_char() -> Result<()> {
        let cb = CursorBuffer::from_str("hello");

        assert_eq!(Location::FindChar(&cb, 'l'), Some(Location::Abs(2)));
        assert_eq!(Location::FindChar(&cb, 'x'), None);
        Ok(())
    }
}
