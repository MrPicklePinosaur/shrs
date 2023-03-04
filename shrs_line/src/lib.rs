//! Readline implementation for shrs

#![feature(linked_list_cursors)]
pub mod completion;
pub mod prompt;

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
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
    }
}

pub fn read_events() -> crossterm::Result<()> {
    let mut buf: LinkedList<u8> = LinkedList::new();
    let mut cursor = buf.cursor_front_mut();

    enable_raw_mode()?;
    loop {
        if poll(Duration::from_millis(1000))? {
            let event = read()?;
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                }) => {
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

            println!("got event {:?}\r", event);
        }
    }
    println!("buffer {:?}\r", buf);
    disable_raw_mode()?;
    Ok(())
}

#[cfg(test)]
mod tests {}
