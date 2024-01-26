use std::{
    cell::RefCell,
    env,
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
    process::Stdio,
    sync::{Arc, OnceLock},
};

use shrs::prelude::*;
use ssh2::{Session, Stream};
use tokio::{
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

struct SshLangCtx {
    stdin_writer: RefCell<BufWriter<Stream>>,
}

impl SshLangCtx {
    // TODO kinda ugly to be passing remote through the original SshLang too
    fn init(runtime: &runtime::Runtime, remote: &str) -> Self {
        // TODO just use env vars for auth for now
        let address = env::var("SHRS_SSH_ADDRESS").unwrap();
        let username = env::var("SHRS_SSH_USERNAME").unwrap();
        let password = env::var("SHRS_SSH_PASSWORD").unwrap();

        let tcp = TcpStream::connect(address).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_password(&username, &password).unwrap();
        // TODO we can implement an interactive password prompt
        // session.userauth_keyboard_interactive();;

        println!("successful auth");

        let mut channel = session.channel_session().unwrap();
        channel.request_pty("xterm", None, None).unwrap();
        channel.shell().unwrap();
        session.set_blocking(false);
        // channel.exec("ls -al").unwrap();

        let _guard = runtime.enter();

        let stdin = channel.stream(0);
        let stdout = channel.stream(0);
        let stderr = channel.stderr();

        runtime.spawn(async {
            let mut stdout_reader = BufReader::new(stdout).lines();
            while let Some(Ok(line)) = stdout_reader.next() {
                println!("{line}");
            }
            eprintln!("EXITED");
        });

        SshLangCtx {
            stdin_writer: RefCell::new(BufWriter::new(stdin)),
        }
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

        // self.runtime.block_on(async {});
        let mut stdin_writer = lang_ctx.stdin_writer.borrow_mut();
        stdin_writer
            .write_all((cmd.clone() + "\n").as_bytes())
            .unwrap();
        stdin_writer.flush().unwrap();
        println!("wrote {cmd}");

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "ssh".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
