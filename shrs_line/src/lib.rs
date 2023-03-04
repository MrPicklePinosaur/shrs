//! Readline implementation for shrs

#![feature(linked_list_cursors)]
#![feature(slice_pattern)]
pub mod completion;
pub mod prompt;

use core::slice::SlicePattern;
use std::{collections::LinkedList, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use prompt::Prompt;

pub struct Line {
    buf: Vec<u8>,
}

impl Line {
    pub fn new() -> Self {
        Line { buf: vec![] }
    }

    pub fn read_line(&self, prompt: &impl Prompt) -> String {
        use std::io::{stdin, stdout, Write};

        // TODO temp prompt (look into terminal repainting)
        print!("{}", prompt.prompt_left());
        stdout().flush().unwrap();

        // get line
        let input = read_events().unwrap();
        println!("[input] {}", input);

        input
    }
}

pub fn read_events() -> crossterm::Result<String> {
    let mut buf: LinkedList<u8> = LinkedList::new();
    let mut cursor = buf.cursor_front_mut();

    enable_raw_mode()?;
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
                    cursor.move_prev();
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                }) => {
                    cursor.move_next();
                },
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: KeyModifiers::NONE,
                }) => {
                    cursor.insert_after(c as u8);
                },
                _ => {},
            }

            // println!("got event {:?}\r", event);
        }
    }
    // println!("buffer {:?}\r", buf);
    disable_raw_mode()?;

    let buf_slice = buf.iter().map(|x| *x).collect::<Vec<_>>();
    let res = std::str::from_utf8(buf_slice.as_slice())
        .unwrap()
        .to_string();
    Ok(res)
}

#[cfg(test)]
mod tests {}
