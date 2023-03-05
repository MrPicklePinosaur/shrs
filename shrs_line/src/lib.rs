//! Readline implementation for shrs

pub mod completion;
pub mod prompt;

use std::{
    collections::LinkedList,
    io::{stdout, BufWriter, Write},
    time::Duration,
};

use crossterm::{
    cursor::{self, position, RestorePosition, SavePosition, Show},
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
        // get line
        let input = self.read_events(prompt).unwrap();

        input
    }

    fn read_events(&self, prompt: &impl Prompt) -> crossterm::Result<String> {
        let mut buf: Vec<u8> = Vec::new();
        let mut ind: i32 = 0;

        let mut painter = Painter::new().unwrap();

        enable_raw_mode()?;

        painter.paint(prompt, "", ind as usize).unwrap();

        loop {
            if poll(Duration::from_millis(1000))? {
                let event = read()?;
                match event {
                    Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        painter.newline()?;
                        break;
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        ind = (ind - 1).max(0);
                    },
                    Event::Key(KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        if !buf.is_empty() {
                            ind = (ind - 1).max(0);
                            buf.remove(ind as usize);
                        }
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
                    },
                    _ => {},
                }

                let buf_cp = buf.clone();

                let res = std::str::from_utf8(buf.as_slice()).unwrap().to_string();

                painter.paint(prompt, &res, ind as usize).unwrap();
            }
        }

        disable_raw_mode()?;

        let res = std::str::from_utf8(buf.as_slice()).unwrap().to_string();
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

    pub fn paint(
        &mut self,
        prompt: &impl Prompt,
        buf: &str,
        cursor_ind: usize,
    ) -> crossterm::Result<()> {
        self.out.queue(cursor::Hide)?;

        // clean up current line first
        self.out
            .queue(cursor::MoveTo(0, self.cursor_pos.1))?
            .queue(Clear(terminal::ClearType::FromCursorDown))?;

        // render line
        self.out
            .queue(Print(prompt.prompt_left()))?
            .queue(Print(&buf[..cursor_ind]))?
            .queue(cursor::SavePosition)?
            .queue(Print(&buf[cursor_ind..]))?;

        self.out.queue(cursor::RestorePosition)?;
        self.out.queue(cursor::Show)?;
        self.out.flush()?;

        Ok(())
    }

    pub fn newline(&mut self) -> crossterm::Result<()> {
        self.out.queue(Print("\r\n"))?;
        self.out.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
