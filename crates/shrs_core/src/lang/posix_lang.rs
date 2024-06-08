use shrs_job::{initialize_job_control, JobManager};
use shrs_lang::{Lexer, Parser, ParserError, Token, PosixError};
use thiserror::Error;

use super::Lang;
use crate::{
    prelude::{CmdOutput, LineContents, States},
    shell::Shell,
};

/// Posix implementation of shell command language
pub struct PosixLang {}

impl Default for PosixLang {
    fn default() -> Self {
        initialize_job_control().unwrap();
        Self {}
    }
}

impl Lang for PosixLang {

    fn eval(&self, _sh: &Shell, states: &States, line: String) -> anyhow::Result<CmdOutput> {
        // TODO rewrite the error handling here better
        // TODO why are we creating a new lexer and parser each eval? is this necessary?
        let lexer = Lexer::new(&line);
        let parser = Parser::default();
        let job_manger = &mut states.get_mut::<JobManager>();

        match shrs_lang::eval(job_manger, parser, lexer) {
            Ok(_) => Ok(CmdOutput::success()),
            Err(_e) => Ok(CmdOutput::error()),
        }
    }

    fn name(&self) -> String {
        "posix".to_string()
    }

    fn needs_line_check(&self, _sh: &Shell, ctx: &States) -> bool {
        //TODO check if open quotes or brackets
        let command = ctx.get::<LineContents>().get_full_command();

        if let Some(last_char) = command.chars().last() {
            if last_char == '\\' {
                return true;
            }
        };

        let mut brackets: Vec<Token> = vec![];

        let lexer = Lexer::new(command.as_str());

        for token in lexer.flatten() {
            match token.1 {
                Token::LBRACE => brackets.push(token.1),
                Token::LPAREN => brackets.push(token.1),
                Token::RPAREN => {
                    if let Some(bracket) = brackets.last() {
                        if bracket == &Token::LPAREN {
                            brackets.pop();
                        } else {
                            return false;
                        }
                    }
                },
                Token::RBRACE => {
                    if let Some(bracket) = brackets.last() {
                        if bracket == &Token::LBRACE {
                            brackets.pop();
                        } else {
                            return false;
                        }
                    }
                },
                Token::WORD(w) => {
                    if let Some(c) = w.chars().next() {
                        if c == '\'' {
                            if w.len() == 1 {
                                return true;
                            }
                            if let Some(e) = w.chars().last() {
                                return e != '\'';
                            } else {
                                return true;
                            }
                        }
                        if c == '\"' {
                            if w.len() == 1 {
                                return true;
                            }

                            if let Some(e) = w.chars().last() {
                                return e != '\"';
                            } else {
                                return true;
                            }
                        }
                    }
                },

                _ => (),
            }
        }

        !brackets.is_empty()
    }
}
