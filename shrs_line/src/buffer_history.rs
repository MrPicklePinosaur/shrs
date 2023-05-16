use shrs_utils::cursor_buffer::{CursorBuffer, Location};

pub trait BufferHistory {
    //redo
    fn next(&mut self, cb: &mut CursorBuffer);
    //undo
    fn prev(&mut self, cb: &mut CursorBuffer);
    //add change to record
    fn add(&mut self, cb: &CursorBuffer);
    //clear all recorded changes
    fn clear(&mut self);
}

// Stores Buffer data and cursor index
struct HistItem(String, usize);

pub struct DefaultBufferHistory {
    index: usize,
    hist: Vec<HistItem>,
}

impl DefaultBufferHistory {
    pub fn new() -> Self {
        DefaultBufferHistory {
            hist: vec![HistItem(String::new(), 0)],
            index: 0,
        }
    }
    fn update_buffer(&mut self, cb: &mut CursorBuffer) {
        cb.set_string(&self.hist[self.index].0);
        cb.move_cursor(Location::Abs(self.hist[self.index].1));
    }
}

impl BufferHistory for DefaultBufferHistory {
    fn next(&mut self, cb: &mut CursorBuffer) {
        if self.index < self.hist.len().saturating_sub(1) {
            self.index += 1;

            self.update_buffer(cb);
        }
    }

    fn prev(&mut self, cb: &mut CursorBuffer) {
        if self.index > 0 {
            self.index = self.index.saturating_sub(1);

            self.update_buffer(cb);
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
