//! Readline implementation for shrs

pub trait Prompt {
    fn prompt_left(&self) -> String;
}

pub struct DefaultPrompt {}

impl DefaultPrompt {
    pub fn new() -> Self {
        DefaultPrompt {}
    }
}

impl Prompt for DefaultPrompt {
    fn prompt_left(&self) -> String {
        String::from("> ")
    }
}

pub struct Line {}

impl Line {
    pub fn new() -> Self {
        Line {}
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

#[cfg(test)]
mod tests {}
