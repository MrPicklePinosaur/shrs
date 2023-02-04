lalrpop_mod!(pub grammar);

use thiserror::Error;

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

    pub fn parse(&mut self, input: &str) -> Result<(), ParserError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::grammar;

    #[test]
    fn parse() {
        assert!(grammar::CommandParser::new().parse("abcd").is_ok());
    }
}
