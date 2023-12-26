//! Generated parser

use thiserror::Error;

use crate::{ast, grammar, lexer::Lexer};

// TODO better errors for unsuccessful parses
#[derive(Error, Debug)]
pub enum Error {
    #[error("unsuccessful parse")]
    UnsuccessfulParse,
}

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(&self, lexer: Lexer) -> Result<ast::Command, Error> {
        grammar::ProgramParser::new()
            .parse(lexer.input(), lexer)
            .map_err(|_e| Error::UnsuccessfulParse)
    }
}

/*
#[cfg(test)]
mod tests {

    use super::grammar;

    #[test]
    fn parse() {
        let res = grammar::ProgramParser::new().parse("ls home | grep downloads");
        println!("{:?}", res);
    }

    #[test]
    fn and_or() {
        let res = grammar::ProgramParser::new().parse("ls home || grep downloads");
        println!("{:?}", res);
    }
}
*/
