use std::cell::RefCell;

use shrs_core::Lang;
use thiserror::Error;

use crate::{eval2, parser, Lexer, Parser};

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
pub struct PosixLang {
    // TODO move to Shell?
    // os: RefCell<Os>,
}

impl PosixLang {
    pub fn new() -> Self {
        // TODO get rid of this unwrap
        // let os = Os::init_shell().unwrap();
        // Self {
        //     os: RefCell::new(os),
        // }
        Self {}
    }
}

impl Lang for PosixLang {
    fn eval(
        &self,
        sh: &shrs_core::Shell,
        ctx: &mut shrs_core::Context,
        rt: &mut shrs_core::Runtime,
        line: String,
    ) -> anyhow::Result<()> {
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
        // let cmd_ctx = process::Context {
        //     stdin: 0,
        //     stdout: 1,
        //     stderr: 2,
        //     is_foreground: true,
        //     is_interactive: true,
        // };
        // let mut os = self.os.borrow_mut();
        // let res = eval2::eval_command(&mut os, &cmd, &cmd_ctx).expect("eval failed");
        // match res {
        //     process::ExitStatus::Exited(status) => println!("exited {status}"),
        //     process::ExitStatus::Running(pid) => println!("running {pid:?}"),
        // }

        Ok(())
    }
}
