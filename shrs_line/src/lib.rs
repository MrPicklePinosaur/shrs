//! Readline implementation for shrs

pub struct Line {}

impl Line {
    pub fn new() -> Self {
        Line {}
    }

    pub fn read_line(&self) -> String {
        use std::io::{stdin, stdout, Write};

        // TODO temp prompt
        print!("> ");
        stdout().flush().unwrap();

        // get line
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
    }
}

#[cfg(test)]
mod tests {}
