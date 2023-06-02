//! Parser for VI like grammar

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod parser;
pub use parser::{Error, Parser};

mod ast;
pub use ast::*;

#[cfg(test)]
mod tests {

    use super::grammar;
    use crate::ast::{Action, Command, Motion};

    #[test]
    fn basic() -> anyhow::Result<()> {
        let res = grammar::CommandParser::new().parse("dw")?;
        assert_eq!(
            res,
            Command {
                repeat: 1,
                action: Action::Delete(Motion::WordPunc)
            }
        );

        let res = grammar::CommandParser::new().parse("42dw")?;
        assert_eq!(
            res,
            Command {
                repeat: 42,
                action: Action::Delete(Motion::WordPunc)
            }
        );

        Ok(())
    }

    #[test]
    fn char_toggle_case() -> anyhow::Result<()> {
        let res = grammar::CommandParser::new().parse("~")?;
        assert_eq!(
            res,
            Command {
                repeat: 1,
                action: Action::ToggleCase
            }
        );
        Ok(())
    }
}
