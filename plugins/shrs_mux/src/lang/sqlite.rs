use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Stdio,
    sync::OnceLock,
};

use shrs::prelude::*;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, Command},
    runtime,
    sync::mpsc::{self, Sender},
};

struct SqliteLangCtx {
    /// Channel for writing to process
    write_tx: Sender<String>,
    _instance: Child,
}

impl SqliteLangCtx {
    fn init(runtime: &runtime::Runtime, db_file: &Path) -> Self {
        let _guard = runtime.enter();

        let args = vec![db_file];
        let mut instance = Command::new("sqlite")
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start sqlite process");

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
                    .expect("Sqlite command failed");

                stdin_writer.flush().await.unwrap();
            }
        });

        Self {
            _instance: instance,
            write_tx,
        }
    }
}

pub struct SqliteLang {
    runtime: runtime::Runtime,
    lang_ctx: OnceLock<SqliteLangCtx>,
    db_file: PathBuf,
}

impl SqliteLang {
    pub fn new(db_file: &Path) -> Self {
        let runtime = runtime::Runtime::new().unwrap();

        Self {
            runtime,
            lang_ctx: OnceLock::new(),
            db_file: db_file.into(),
        }
    }
}

impl Lang for SqliteLang {
    fn eval(
        &self,
        _sh: &Shell,
        _ctx: &mut Context,
        _rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let lang_ctx = self
            .lang_ctx
            .get_or_init(|| SqliteLangCtx::init(&self.runtime, &self.db_file));

        self.runtime.block_on(async {
            lang_ctx.write_tx.send(cmd).await.unwrap();
        });

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "sqlite".to_string()
    }

    fn needs_line_check(&self, _state: &LineStateBundle) -> bool {
        false
    }
}
