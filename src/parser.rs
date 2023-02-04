use pest::{iterators::Pair, Parser};

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

        self.parse_root(parsed)?;
        Ok(())
    }

    fn parse_root(&mut self, pair: Pair<Rule>) -> Result<(), ParserError> {
        // TODO find if we can build in this assert into the signature of the function
        assert!(pair.as_rule() == Rule::root);

        for command in pair.into_inner() {
            if command.as_rule() == Rule::command {
                self.parse_command(command);
            }
        }

        Ok(())
    }

    fn parse_command(&mut self, pair: Pair<Rule>) -> Result<(), ParserError> {
        assert!(pair.as_rule() == Rule::command);

        let mut it = pair.into_inner();
        let cmd = it.next().unwrap();
        println!("cmd {:?}", cmd);
        for arg in it {
            println!("arg {:?}", arg);
        }
        Ok(())
    }
}
