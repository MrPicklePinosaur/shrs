use std::{
    cell::RefCell,
    env,
    net::TcpStream,
    sync::{Arc, OnceLock},
};

use openssh::{Child, KnownHosts, Session, Stdio};
use shrs::prelude::*;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter},
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

struct SshLangCtx {
    session: Arc<Session>,
    shell: Child<Arc<Session>>,
}

impl SshLangCtx {
    // TODO kinda ugly to be passing remote through the original SshLang too
    fn init(runtime: &runtime::Runtime, remote: &str) -> Self {
        let _guard = runtime.enter();

        let ctx = runtime.block_on(async {
            let session =
                Session::connect(env::var("SHRS_SSH_ADDRESS").unwrap(), KnownHosts::Strict)
                    .await
                    .unwrap();
            let session = Arc::new(session);

            println!("connected to server");

            let mut shell = session
                .clone()
                .arc_command("sh")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .await
                .unwrap();

            let stdout = shell.stdout().take().unwrap();

            runtime.spawn(async {
                let mut stdout_reader = BufReader::new(stdout).lines();
                while let Some(line) = stdout_reader.next_line().await.unwrap() {
                    println!("{line}");
                }
            });

            SshLangCtx { session, shell }
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
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let lang_ctx = self
            .lang_ctx
            .get_or_init(|| SshLangCtx::init(&self.runtime, &self.remote));

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "ssh".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
