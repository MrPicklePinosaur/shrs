use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub enum Command {
    /// Basic command
    ///
    /// ```sh
    /// ls -al
    /// ```
    Simple(Vec<Word>),

    /// Two commands joined by a pipe
    ///
    /// ```sh
    /// cat .bashrc | wc -l
    /// ```
    Pipeline(Box<Command>, Box<Command>),
}

#[derive(Debug)]
pub struct Word(pub String);

impl Deref for Word {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Word {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
