use thiserror::Error;

use crate::{ast, grammar::CommandParser};

// TODO better errors for unsuccessful parses
#[derive(Error, Debug)]
pub enum Error {
    #[error("unsuccessful parse")]
    UnsuccessfulParse,
}

pub struct Parser {
    parser: CommandParser,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            parser: CommandParser::new(),
        }
    }
}

impl Parser {
    pub fn parse(&mut self, input: &str) -> Result<ast::Command, Error> {
        self.parser
            .parse(input)
            .map_err(|_| Error::UnsuccessfulParse)
    }
}
