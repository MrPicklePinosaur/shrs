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

pub struct BashLang {
    instance: RefCell<Child>,
}

impl BashLang {
    pub fn new() -> Self {
        Self {
            instance: RefCell::new(
                Command::new("bash")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to start bash process"),
            ),
        }
    }
}

impl Lang for BashLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut instance = self.instance.borrow_mut();
        let stdin = instance.stdin.as_mut().expect("Failed to open stdin");

        let cd_statement = format!("cd {}\n", rt.working_dir.to_string_lossy());

        stdin
            .write_all(cd_statement.as_bytes())
            .expect("unable to set var");

        for (k, v) in rt.env.iter() {
            let export_statement = format!("export {}={:?}\n", k, v);
            stdin
                .write_all(export_statement.as_bytes())
                .expect("unable to set var");
        }
        stdin
            .write_all((cmd + ";echo $?'\x1A'; echo '\x1A' >&2\n").as_bytes())
            .expect("Bash command failed");

        let stdout_reader =
            BufReader::new(instance.stdout.as_mut().expect("Failed to open stdout"));
        let status = read_out(ctx, stdout_reader)?;

        let stderr_reader =
            BufReader::new(instance.stderr.as_mut().expect("Failed to open stdout"));
        read_err(ctx, stderr_reader)?;

        Ok(CmdOutput::new(status))
    }

    fn name(&self) -> String {
        "bash".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
