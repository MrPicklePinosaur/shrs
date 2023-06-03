use shrs_core::{
    lang::Lang,
    shell::{Context, Runtime, Shell},
};
use shrs_job::initialize_job_control;
use thiserror::Error;

use crate::{
    eval2::{self, run_job},
    parser, Lexer, Parser,
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
        _ctx: &mut Context,
        _rt: &mut Runtime,
        line: String,
    ) -> anyhow::Result<()> {
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
        println!("{cmd:?}");

        let mut job_manager = sh.job_manager.borrow_mut();
        let (procs, pgid) = eval2::eval_command(&mut job_manager, &cmd, None, None)?;

        run_job(&mut job_manager, procs, pgid, true)?;

        Ok(())
    }
}
