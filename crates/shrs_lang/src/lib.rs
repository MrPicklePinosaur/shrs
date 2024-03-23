//! POSIX shell lexer and parser
//!
//!

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod parser;
pub use parser::{Parser, ParserError};

mod lexer;
pub use lexer::{Lexer, Token, RESERVED_WORDS};

pub mod ast;

// pub mod process;
// pub mod eval;

// pub mod process;
