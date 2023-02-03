use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ShParser;

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
        let parsed = ShParser::parse(Rule::root, input)
            .map_err(|e| ParserError::UnsuccessfulParse(e.to_string()))?
            .next()
            .unwrap();

        Ok(())
    }
}
