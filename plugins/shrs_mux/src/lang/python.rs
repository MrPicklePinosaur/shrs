use std::{
    io::Write,
    process::Stdio,
    sync::{Arc, OnceLock},
};

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

struct PythonLangCtx {
    /// Channel for writing to process
    write_tx: Sender<String>,
    instance: Child,
}

impl PythonLangCtx {
    fn init(runtime: &runtime::Runtime) -> Self {
        let _guard = runtime.enter();

        // TODO maybe support custom parameters to pass to command
        // pass some options to make repl work better
        // -i forces interactive
        // -q silences help message
        // the command given by the -c is used to remove the prompt
        let args = vec!["-i", "-q", "-c", "import sys; sys.ps1=''; sys.ps2=''"];
        let mut instance = Command::new("python")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start python process");

        let stdout = instance.stdout.take().unwrap();
        let stderr = instance.stderr.take().unwrap();
        let stdin = instance.stdin.take().unwrap();

        runtime.spawn(async {
            let mut stdout_reader = BufReader::new(stdout).lines();
            while let Some(line) = stdout_reader.next_line().await.unwrap() {
                write!(std::io::stdout(), "{line}\r\n").unwrap();
            }
        });

        runtime.spawn(async {
            let mut stderr_reader = BufReader::new(stderr).lines();
            while let Some(line) = stderr_reader.next_line().await.unwrap() {
                write!(std::io::stderr(), "{line}\r\n").unwrap();
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

        Self { instance, write_tx }
    }
}

pub struct PythonLang {
    runtime: runtime::Runtime,
    lang_ctx: OnceLock<PythonLangCtx>,
}

impl PythonLang {
    pub fn new() -> Self {
        let runtime = runtime::Runtime::new().unwrap();

        Self {
            runtime,
            lang_ctx: OnceLock::new(),
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
        let lang_ctx = self
            .lang_ctx
            .get_or_init(|| PythonLangCtx::init(&self.runtime));

        self.runtime.block_on(async {
            lang_ctx.write_tx.send(cmd).await.unwrap();
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
