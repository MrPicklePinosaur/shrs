use std::{
    cell::RefCell,
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    ops::Add,
    os::unix::process::ExitStatusExt,
    process::{Child, ChildStderr, ChildStdout, Command, ExitStatus, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use shrs::prelude::*;

use crate::{
    interpreter::{read_err, read_out},
    MuxState,
};

pub struct MuxLang {
    langs: HashMap<String, Box<dyn Lang>>,
}

impl MuxLang {
    pub fn new(langs: HashMap<String, Box<dyn Lang>>) -> Self {
        // TODO should be configurable later
        Self { langs }
    }
}

impl Lang for MuxLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> anyhow::Result<CmdOutput> {
        let lang_name = match ctx.state.get::<MuxState>() {
            Some(state) => &state.lang,
            None => return Ok(CmdOutput::empty()),
        };
        // TODO maybe return error if we can't find a lang

        if let Some(lang) = self.langs.get(lang_name) {
            return lang.eval(sh, ctx, rt, cmd);
        }

        Ok(CmdOutput::empty())
    }

    fn name(&self) -> String {
        "mux".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}

pub struct NuLang {}

impl NuLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for NuLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut handle = Command::new("nu")
            .args(vec!["-c", &cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = handle.wait_with_output()?;

        Ok(CmdOutput::from(output))
    }

    fn name(&self) -> String {
        "nu".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}

pub struct PythonLang {}

impl PythonLang {
    pub fn new() -> Self {
        Self {}
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
        let mut handle = Command::new("python3")
            .args(vec!["-c", &cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = handle.wait_with_output()?;

        Ok(CmdOutput::from(output))
    }

    fn name(&self) -> String {
        "python".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}

pub struct BashLang {
    pub instance: Arc<Mutex<Child>>,
}

impl BashLang {
    pub fn new() -> Self {
        Self {
            instance: Arc::new(Mutex::new(
                Command::new("bash")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to start bash lol"),
            )),
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
        {
            let mut guard = self.instance.lock().unwrap();

            let stdin = guard.stdin.as_mut().expect("Failed to open stdin");

            stdin.write_all((cmd + ";echo $?'\x1A'; echo '\x1A' >&2\n").as_bytes())?;
        }

        let err_inst = self.instance.clone();
        let out_inst = self.instance.clone();
        let stdout_thread = thread::spawn(move || read_out(out_inst));
        let stderr_thread = thread::spawn(move || read_err(err_inst));

        let stderr = stderr_thread.join().unwrap()?;
        let (stdout, status) = stdout_thread.join().unwrap()?;

        Ok(CmdOutput::new(stdout, stderr, ExitStatus::from_raw(status)))
    }

    fn name(&self) -> String {
        "bash".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
