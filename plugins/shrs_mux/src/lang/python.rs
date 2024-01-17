use std::{process::Stdio, sync::Arc};

use shrs::prelude::*;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, ChildStdin, ChildStdout, Command},
    runtime,
    sync::{
        mpsc::{self, Sender},
        RwLock,
    },
};

use crate::{
    interpreter::{read_err, read_out},
    MuxState,
};

pub struct PythonLang {
    instance: Child,
    /// Channel for writing to process
    write_tx: Sender<String>,
    runtime: runtime::Runtime,
}

impl PythonLang {
    pub fn new() -> Self {
        let runtime = runtime::Runtime::new().unwrap();

        let _guard = runtime.enter();

        // TODO maybe support custom parameters to pass to command
        let mut instance = Command::new("python")
            .arg("-i")
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

        let (write_tx, mut write_rx) = mpsc::channel::<String>(8);

        runtime.spawn(async move {
            let mut stdin_writer = BufWriter::new(stdin);

            while let Some(cmd) = write_rx.recv().await {
                stdin_writer
                    .write_all((cmd + "\n").as_bytes())
                    .await
                    .expect("Python command failed");

                stdin_writer.flush().await.unwrap();
            }
        });

        Self {
            instance,
            write_tx,
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
        self.runtime.block_on(async {
            self.write_tx.send(cmd).await.unwrap();
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
