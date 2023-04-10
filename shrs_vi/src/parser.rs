use thiserror::Error;

use crate::{ast, grammar};

// TODO better errors for unsuccessful parses
#[derive(Error, Debug)]
pub enum Error {
    #[error("unsuccessful parse")]
    UnsuccessfulParse,
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&mut self, input: &str) -> Result<ast::Command, Error> {
        grammar::CommandParser::new()
            .parse(input)
            .map_err(|e| Error::UnsuccessfulParse)
    }
}
