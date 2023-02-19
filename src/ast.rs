use std::ops::{Deref, DerefMut};

use pino_deref::Deref;

#[derive(Debug)]
pub struct Redirect {
    pub n: Option<IONumber>,
    pub file: Filename,
    pub mode: RedirectMode,
}

#[derive(Debug)]
pub enum RedirectMode {
    Read,
    Write,
    ReadAppend,
    WriteAppend,
    ReadDup,
    WriteDup,
    ReadWrite,
}

#[derive(Debug)]
pub enum Command {
    /// Basic command
    ///
    /// ```sh
    /// ls -al
    /// ```
    Simple {
        redirects: Vec<Redirect>,
        args: Vec<Word>,
    },

    /// Two commands joined by a pipe
    ///
    /// ```sh
    /// cat .bashrc | wc -l
    /// ```
    Pipeline(Box<Command>, Box<Command>),

    /// Compound command of And
    And(Box<Command>, Box<Command>),

    /// Compound command of Or
    Or(Box<Command>, Box<Command>),

    Not(Box<Command>),
}

#[derive(Debug)]
pub struct Word(pub String);

#[derive(Deref, Debug)]
pub struct Filename(pub String);

#[derive(Deref, Debug)]
pub struct IONumber(pub usize);

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
