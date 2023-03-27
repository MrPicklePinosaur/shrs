//! POSIX shell lexer and parser
//!
//!

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod parser;
pub use parser::{Error, Parser};

mod lexer;
pub use lexer::{Lexer, RESERVED_WORDS};

pub mod ast;
