//! Readline implementation for shrs

#![feature(linked_list_cursors)]
#![feature(slice_pattern)]
pub mod completion;
pub mod prompt;

use core::slice::SlicePattern;
use std::{
    collections::LinkedList,
    io::{stdout, BufWriter, Write},
    time::Duration,
};

use crossterm::{
    cursor::{self, position, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::Print,
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear},
    ExecutableCommand, QueueableCommand,
};
use prompt::Prompt;

pub struct Line {}

impl Line {
    pub fn new() -> Self {
        Line {}
    }

    pub fn read_line(&self, prompt: &impl Prompt) -> String {
        use std::io::{stdin, stdout, Write};

        // TODO temp prompt (look into terminal repainting)
        // print!("{}", prompt.prompt_left());
        // stdout().flush().unwrap();

        // get line
        let input = self.read_events().unwrap();
        println!("[input] {}", input);

        input
    }

    fn read_events(&self) -> crossterm::Result<String> {
        let mut buf: Vec<u8> = Vec::new();
        let mut ind: i32 = 0;
        // let mut cursor = buf.cursor_front_mut();

        let mut painter = Painter::new().unwrap();

        enable_raw_mode()?;
        execute!(stdout(), Show);

        painter.paint("").unwrap();

        loop {
            if poll(Duration::from_millis(1000))? {
                let event = read()?;
                match event {
                    Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        // accept current input
                        break;
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        ind = (ind - 1).max(0);
                        // cursor.move_prev();
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        ind = if buf.is_empty() {
                            0
                        } else {
                            (ind + 1).min(buf.len() as i32)
                        };
                        // cursor.move_next();
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Char(c),
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        buf.insert(ind as usize, c as u8);
                        ind = if buf.is_empty() {
                            0
                        } else {
                            (ind + 1).min(buf.len() as i32)
                        };
                        // cursor.insert_before(c as u8);
                    },
                    _ => {},
                }

                let buf_cp = buf.clone();

                // TODO dup code
                let buf_slice = buf.iter().map(|x| *x).collect::<Vec<_>>();
                let res = std::str::from_utf8(buf_slice.as_slice())
                    .unwrap()
                    .to_string();

                painter.paint(&res).unwrap();

                // println!("got event {:?}\r", event);
            }
        }
        // println!("buffer {:?}\r", buf);
        // painter.paint(&res).unwrap();
        let buf_slice = buf.iter().map(|x| *x).collect::<Vec<_>>();
        let res = std::str::from_utf8(buf_slice.as_slice())
            .unwrap()
            .to_string();

        disable_raw_mode()?;

        Ok(res)
    }
}

struct Painter {
    /// The output buffer
    out: BufWriter<std::io::Stdout>,
    /// Dimensions of current terminal window
    term_size: (u16, u16),
    /// Position of the cursor
    cursor_pos: (u16, u16),
}

impl Painter {
    pub fn new() -> crossterm::Result<Self> {
        let term_size = terminal::size()?;
        let cursor_pos = cursor::position()?;
        Ok(Painter {
            out: BufWriter::new(stdout()),
            term_size,
            cursor_pos,
        })
    }

    fn paint(&mut self, buf: &str) -> crossterm::Result<()> {
        self.out.queue(cursor::Hide)?;

        self.out.queue(cursor::MoveTo(0, self.cursor_pos.1))?;
        self.out.queue(Clear(terminal::ClearType::FromCursorDown))?;

        self.out.queue(Print(">> "))?;
        self.out.queue(Print(buf))?;

        self.out.queue(cursor::Show)?;
        self.out.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
