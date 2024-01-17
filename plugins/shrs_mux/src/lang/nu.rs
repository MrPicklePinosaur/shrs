use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::format,
    io::{BufRead, BufReader, Read, Write},
    ops::Add,
    os::unix::process::ExitStatusExt,
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Stdio},
    sync::Arc,
};

use shrs::{
    lang::{Lexer, Token},
    prelude::*,
};

use crate::{
    interpreter::{read_err, read_out},
    MuxState,
};

pub struct NuLang {}

impl NuLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for NuLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut handle = Command::new("nu")
            .args(vec!["-c", &cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = handle.wait_with_output()?;
        // ctx.out.print(output.stdout);

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "nu".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
