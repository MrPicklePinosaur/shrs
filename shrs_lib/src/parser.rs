use thiserror::Error;

use crate::{ast, grammar, lexer::Lexer};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unsuccessful parse")]
    UnsuccessfulParse,
}

pub struct ParserContext {}

impl ParserContext {
    pub fn new() -> Self {
        ParserContext {}
    }

    pub fn parse(&mut self, lexer: Lexer) -> Result<ast::Command, ParserError> {
        grammar::ProgramParser::new()
            .parse(lexer.input(), lexer)
            .map_err(|e| ParserError::UnsuccessfulParse)
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
