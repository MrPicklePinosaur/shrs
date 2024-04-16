use shrs_utils::cursor_buffer::{self, CursorBuffer, Location};

pub trait BufferHistory {
    /// Redo
    fn next(&mut self, cb: &mut CursorBuffer);
    /// Undo
    fn prev(&mut self, cb: &mut CursorBuffer);
    /// Add change to record
    fn add(&mut self, cb: &CursorBuffer);
    /// Clear all recorded changes
    fn clear(&mut self);
}

// Stores Buffer data and cursor index
#[derive(Default)]
struct HistItem(String, usize);

pub struct DefaultBufferHistory {
    index: usize,
    hist: Vec<HistItem>,
}

impl Default for DefaultBufferHistory {
    fn default() -> Self {
        DefaultBufferHistory {
            hist: vec![HistItem(String::new(), 0)],
            index: 0,
        }
    }
}

impl DefaultBufferHistory {
    fn update_buffer(&mut self, cb: &mut CursorBuffer) -> cursor_buffer::Result<()> {
        let new_buf = &self.hist.get(self.index).unwrap().0;
        cb.clear();
        cb.insert(Location::Cursor(), new_buf)?;
        cb.move_cursor(Location::Abs(self.hist[self.index].1))?;
        Ok(())
    }
}

impl BufferHistory for DefaultBufferHistory {
    fn next(&mut self, cb: &mut CursorBuffer) {
        if self.index < self.hist.len().saturating_sub(1) {
            self.index += 1;

            self.update_buffer(cb).unwrap();
        }
    }

    fn prev(&mut self, cb: &mut CursorBuffer) {
        if self.index > 0 {
            self.index = self.index.saturating_sub(1);

            self.update_buffer(cb).unwrap();
        }
    }

    fn add(&mut self, cb: &CursorBuffer) {
        //if change occurs while undoing remove all changes after current index
        if !self.hist.is_empty() && self.index != self.hist.len() {
            self.hist.drain((self.index + 1)..);
        }
        let b: String = cb.chars(Location::Front()).unwrap().collect();
        //only record change if there is a difference in data
        if let Some(last) = self.hist.last() {
            if b == last.0 {
                return;
            }
        }
        self.hist.push(HistItem(b, cb.cursor()));
        self.index += 1;
    }
    fn clear(&mut self) {
        self.hist.drain(1..);
        self.index = 0;
    }
}
