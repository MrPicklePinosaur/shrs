lalrpop_mod!(pub grammar);

use thiserror::Error;

use crate::ast;

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
        grammar::PipeSequenceParser::new()
            .parse(input)
            .map_err(|e| ParserError::UnsuccessfulParse(e.to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::grammar;

    #[test]
    fn parse() {
        let res = grammar::AndOrParser::new().parse("ls home | grep downloads");
        println!("{:?}", res);
    }
}
