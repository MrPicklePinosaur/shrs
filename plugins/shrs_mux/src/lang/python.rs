use std::{process::Stdio, sync::Arc};

use shrs::prelude::*;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout, Command},
    runtime,
    sync::RwLock,
};

use crate::{
    interpreter::{read_err, read_out},
    MuxState,
};

pub struct PythonLang {
    instance: Child,
    stdin: Arc<RwLock<ChildStdin>>,
    runtime: runtime::Runtime,
}

impl PythonLang {
    pub fn new() -> Self {
        let runtime = runtime::Runtime::new().unwrap();

        let _guard = runtime.enter();

        // TODO maybe support custom parameters to pass to command
        let mut instance = Command::new("python")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start python process");

        let stdin = instance.stdin.take().unwrap();
        let stdout = instance.stdout.take().unwrap();

        runtime.spawn(async {
            let mut stdout_reader = BufReader::new(stdout).lines();
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
            let mut stdin_writer = BufWriter::new(&mut *borrow);
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
