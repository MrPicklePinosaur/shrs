//! Readline implementation for shrs
pub mod completion;
pub mod prompt;

use std::time::Duration;

use crossterm::{
    event::{poll, read, Event, KeyCode},
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
    let mut point = 0;
    let mut buf: Vec<u8> = vec![];

    enable_raw_mode()?;
    loop {
        if poll(Duration::from_millis(1000))? {
            let event = read()?;
            println!("got event {:?}\r", event);
            match event {
                Event::Key(keycode) => {
                    // handle special characters
                    if keycode == KeyCode::Esc.into() {
                        break;
                    }

                    if keycode == KeyCode::Left {}
                    if keycode == KeyCode::Right {}

                    // otherwise we can append inputted character to buffer
                },
                _ => {},
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}

#[cfg(test)]
mod tests {}
