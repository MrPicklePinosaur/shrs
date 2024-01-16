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
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    runtime,
    sync::RwLock,
};

use crate::{
    interpreter::{read_err, read_out},
    MuxState,
};

pub struct PythonLang {
    instance: tokio::process::Child,
    stdin: Arc<RwLock<tokio::process::ChildStdin>>,
    runtime: runtime::Runtime,
}

impl PythonLang {
    pub fn new() -> Self {
        // TODO maybe support custom parameters to pass to command
        let mut instance = tokio::process::Command::new("python")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start python process");

        let stdout = instance.stdout.take().unwrap();
        let stdin = instance.stdin.take().unwrap();

        let runtime = runtime::Runtime::new().unwrap();

        runtime.spawn(async {
            let mut stdout_reader = tokio::io::BufReader::new(stdout).lines();
            while let Some(line) = stdout_reader.next_line().await.unwrap() {
                println!("{line}");
            }
        });

        Self {
            instance,
            stdin: Arc::new(RwLock::new(stdin)),
            runtime,
        }
    }
}

impl Lang for PythonLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let stdin_clone = Arc::clone(&self.stdin);

        self.runtime.spawn(async move {
            let mut borrow = stdin_clone.write().await;
            let mut stdin_writer = tokio::io::BufWriter::new(&mut *borrow);
            stdin_writer
                .write_all((cmd + "\n").as_bytes())
                .await
                .expect("Python command failed");
        });

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "python".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
