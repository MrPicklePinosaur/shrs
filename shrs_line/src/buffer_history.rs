use std::fs::File;
use std::io::Write;

use shrs_utils::cursor_buffer::{CursorBuffer, Location};

pub trait BufferHistory {
    fn next(&mut self, cb: &mut CursorBuffer);
    fn prev(&mut self, cb: &mut CursorBuffer);
    fn add(&mut self, cb: &CursorBuffer);
    fn clear(&mut self);
}

pub struct DefaultBufferHistory {
    index: usize,
    hist: Vec<(String, usize)>,
    // w: File,
}

impl DefaultBufferHistory {
    pub fn new() -> Self {
        DefaultBufferHistory {
            hist: vec![],
            index: 0,
            // w: File::create("./debug.txt").unwrap(),
        }
    }
    fn update_buffer(&mut self, cb: &mut CursorBuffer) {
        cb.set_string(&self.hist[self.index].0);
        cb.move_cursor(Location::Abs(self.hist[self.index].1));
    }
}

impl BufferHistory for DefaultBufferHistory {
    fn next(&mut self, cb: &mut CursorBuffer) {
        if self.index < self.hist.len() - 1 {
            self.index += 1;
            self.update_buffer(cb);
            // writeln!(
            //     self.w,
            //     "{:?}",
            //     self.hist
            //         .iter()
            //         .map(|h| { h.0.clone() })
            //         .collect::<Vec<String>>()
            // )
            // .unwrap();
        }
    }

    fn prev(&mut self, cb: &mut CursorBuffer) {
        if self.index > 0 {
            self.index -= 1;
            self.update_buffer(cb);
            // writeln!(
            //     self.w,
            //     "{:?}",
            //     self.hist
            //         .iter()
            //         .map(|h| { h.0.clone() })
            //         .collect::<Vec<String>>()
            // )
            // .unwrap();
        }
    }

    fn add(&mut self, cb: &CursorBuffer) {
        // writeln!(self.w, "adding");

        if !self.hist.is_empty() && self.index != self.hist.len() {
            self.hist.drain((self.index + 1)..);
        }
        let b: String = cb.chars(Location::Front()).unwrap().collect();
        if let Some(last) = self.hist.last() {
            if b == last.0 {
                return;
            }
        }
        self.hist.push((b, cb.cursor()));
        self.index += 1;
    }
    fn clear(&mut self) {
        self.hist.clear();
        self.index = 0;
    }
}
