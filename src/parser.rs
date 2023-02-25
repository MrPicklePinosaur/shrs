use thiserror::Error;

use crate::{ast, grammar};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("unsuccessful parse")]
    UnsuccessfulParse(String),
}

pub struct ParserContext {}

impl ParserContext {
    pub fn new() -> Self {
        ParserContext {}
    }

    pub fn parse(&mut self, input: &str) -> Result<ast::Command, ParserError> {
        grammar::ProgramParser::new()
            .parse(input)
            .map_err(|e| ParserError::UnsuccessfulParse(e.to_string()))
    }
}

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
