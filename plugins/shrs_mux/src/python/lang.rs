use std::{
    process::Stdio,
    sync::{Arc, OnceLock},
};

use shrs::prelude::{styled_buf::StyledBuf, *};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    process::{Child, Command},
    runtime,
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
};

use super::error_theme::PythonErrorTheme;

struct PythonLangCtx {
    /// Channel for writing to process
    write_tx: Sender<String>,
    _instance: Child,
}

impl PythonLangCtx {
    fn init(runtime: &runtime::Runtime) -> Self {
        let _guard = runtime.enter();
        let out = Arc::new(Mutex::new(OutputWriter::default()));
        let err_theme = PythonErrorTheme::new();
        let err = out.clone();

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

        runtime.spawn(async move {
            let mut stdout_reader = BufReader::new(stdout).lines();
            while let Some(line) = stdout_reader.next_line().await.unwrap() {
                let mut guard = out.lock().await;

                guard.println(format!("{line}")).unwrap();
            }
        });

        runtime.spawn(async move {
            let mut stderr_reader = BufReader::new(stderr).lines();
            while let Some(line) = stderr_reader.next_line().await.unwrap() {
                let mut o = err.lock().await;
                let mut buf = StyledBuf::new(line.as_str());
                err_theme.apply(&mut buf);

                o.print_buf(buf).unwrap();
                o.println("").unwrap();
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
            _instance: instance,
            write_tx,
        }
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
    fn eval(&self, _sh: &Shell, _states: &States, cmd: String) -> shrs::anyhow::Result<CmdOutput> {
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

    fn needs_line_check(&self, _shell: &Shell, _ctx: &States) -> bool {
        false
    }
}
