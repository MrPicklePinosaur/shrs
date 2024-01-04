use shrs_core::{
    lang::Lang,
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};
use shrs_job::initialize_job_control;
use thiserror::Error;

// use crate::eval::{command_output, eval_command},
use crate::{eval2, parser, Lexer, Parser, Token};

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
    /// Command not found
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    /// Job manager specific error
    #[error("Job manager error: {0}")]
    Job(anyhow::Error),
}

/// Posix implementation of shell command language
pub struct PosixLang {}

impl Default for PosixLang {
    fn default() -> Self {
        initialize_job_control().unwrap();
        Self {}
    }
}

impl Lang for PosixLang {
    /* eval1 impl
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        line: String,
    ) -> anyhow::Result<CmdOutput> {
        // TODO rewrite the error handling here better
        let lexer = Lexer::new(&line);
        let mut parser = Parser::new();
        let cmd = match parser.parse(lexer) {
            Ok(cmd) => cmd,
            Err(e) => {
                // TODO detailed parse errors
                eprintln!("{e}");
                return Err(e.into());
            },
        };
        let exit_status =
            match eval_command(sh, ctx, rt, &cmd, Stdio::inherit(), Stdio::inherit(), None) {
                Ok(cmd_handle) => cmd_handle,
                Err(e) => {
                    eprintln!("{e}");
                    return Err(e);
                },
            };

        match exit_status {
            crate::process::ExitStatus::Exited(_) => {},
            crate::process::ExitStatus::Running(pid) => {},
        }

        // TODO make this accurate
        Ok(CmdOutput::success())
    }
    */

    fn eval(
        &self,
        sh: &Shell,
        _ctx: &mut Context,
        _rt: &mut Runtime,
        line: String,
    ) -> anyhow::Result<CmdOutput> {
        // TODO rewrite the error handling here better
        let lexer = Lexer::new(&line);
        let parser = Parser::default();
        let cmd = match parser.parse(lexer) {
            Ok(cmd) => cmd,
            Err(e) => {
                // TODO detailed parse errors
                eprintln!("parse error: {e}");
                return Err(e.into());
            },
        };

        let mut job_manager = sh.job_manager.borrow_mut();
        let (procs, pgid) = eval2::eval_command(&mut job_manager, &cmd, None, None)?;

        eval2::run_job(&mut job_manager, procs, pgid, true)?;

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
