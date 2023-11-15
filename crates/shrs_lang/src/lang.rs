use std::{os::unix::process::ExitStatusExt, process::ExitStatus};

use shrs_core::{
    lang::Lang,
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};
use shrs_job::{initialize_job_control, Output};
use thiserror::Error;

use crate::{
    eval2::{self, run_job},
    parser, Lexer, Parser, Token,
};

#[derive(Error, Debug)]
pub enum PosixError {
    /// Error when attempting file redirection
    #[error("Redirection Error: {0}")]
    Redirect(std::io::Error),
    /// Error emitted by hook
    #[error("Hook Error:")]
    Hook(),
    /// Issue parsing command
    #[error("Parse failed: {0}")]
    Parse(parser::Error),
    /// Issue evaluating command
    #[error("Failed evaluating command: {0}")]
    Eval(anyhow::Error),
}

/// Posix implementation of shell command language
pub struct PosixLang {}

impl PosixLang {
    pub fn new() -> Self {
        initialize_job_control().unwrap();
        Self {}
    }
}

impl Lang for PosixLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        line: String,
    ) -> anyhow::Result<CmdOutput> {
        // TODO rewrite the error handling here better
        let lexer = Lexer::new(&line);
        let parser = Parser::new();
        let cmd = match parser.parse(lexer) {
            Ok(cmd) => cmd,
            Err(e) => {
                // TODO detailed parse errors
                eprintln!("{e}");
                return Err(e.into());
            },
        };

        let mut job_manager = sh.job_manager.borrow_mut();
        let (procs, pgid) = eval2::eval_command(&mut job_manager, &cmd, None, None)?;

        run_job(&mut job_manager, procs, pgid, true)?;

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "posix".to_string()
    }
    fn needs_line_check(&self, command: String) -> bool {
        //TODO check if open quotes or brackets

        if let Some(last_char) = command.chars().last() {
            if last_char == '\\' {
                return true;
            }
        };

        let mut brackets: Vec<Token> = vec![];

        let lexer = Lexer::new(command.as_str());

        for t in lexer {
            if let Ok(token) = t {
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
        }

        !brackets.is_empty()
    }
}
