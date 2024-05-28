use std::{
    io::Write,
    sync::{Arc, OnceLock},
};

use openssh::{Child, KnownHosts, Session, Stdio};
use shrs::prelude::*;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
    runtime,
    sync::mpsc::{self, Sender},
};

#[allow(unused)]
struct SshLangCtx {
    session: Arc<Session>,
    shell: Child<Arc<Session>>,
    write_tx: Sender<String>,
}

impl SshLangCtx {
    // TODO kinda ugly to be passing remote through the original SshLang too
    fn init(runtime: &runtime::Runtime, remote: &str) -> Self {
        let _guard = runtime.enter();

        let ctx = runtime.block_on(async {
            let session = Session::connect(remote, KnownHosts::Strict).await.unwrap();
            let session = Arc::new(session);

            println!("connected to server");

            let mut shell = session
                .clone()
                .arc_command("bash") // TODO let user customize the login shell
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .await
                .unwrap();

            let stdout = shell.stdout().take().unwrap();
            let stderr = shell.stderr().take().unwrap();
            let stdin = shell.stdin().take().unwrap();

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
                        .expect("Ssh command failed");

                    stdin_writer.flush().await.unwrap();
                }
            });

            SshLangCtx {
                session,
                shell,
                write_tx,
            }
        });

        ctx
    }
}

pub struct SshLang {
    runtime: runtime::Runtime,
    lang_ctx: OnceLock<SshLangCtx>,
    remote: String,
}

impl SshLang {
    pub fn new(remote: impl ToString) -> Self {
        let runtime = runtime::Runtime::new().unwrap();

        Self {
            runtime,
            lang_ctx: OnceLock::new(),
            remote: remote.to_string(),
        }
    }
}

impl Lang for SshLang {
    fn eval(&self, _sh: &Shell, _states: &States, cmd: String) -> shrs::anyhow::Result<CmdOutput> {
        let lang_ctx = self
            .lang_ctx
            .get_or_init(|| SshLangCtx::init(&self.runtime, &self.remote));

        self.runtime.block_on(async {
            lang_ctx.write_tx.send(cmd).await.unwrap();
        });

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "ssh".to_string()
    }

    fn needs_line_check(&self, _shell: &Shell, _ctx: &States) -> bool {
        false
    }
}
