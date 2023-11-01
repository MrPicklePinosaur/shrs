use std::{
    cell::RefCell,
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    ops::Add,
    process::{Child, Command, ExitStatus, Stdio},
};

use shrs::prelude::*;

use crate::MuxState;

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
    pub instance: RefCell<Child>,
}

impl BashLang {
    pub fn new() -> Self {
        Self {
            instance: RefCell::new(
                Command::new("bash")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to start bash lol"),
            ),
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
        let mut instance = self.instance.borrow_mut();
        let stdin = instance.stdin.as_mut().expect("Failed to open stdin");
        // stdin
        //     .write_all(cmd.as_bytes())
        //     .expect("Failed to write to stdin");

        //idk how to echo to stdout and stderr
        stdin
            .write_all((cmd + ";echo '\x1A'; echo '\x1A' >&2\n").as_bytes())
            .expect("Failed to send Ctrl+C to stdin");

        let mut stdout_reader =
            BufReader::new(instance.stdout.as_mut().expect("Failed to open stdout"));

        let mut stdout = String::new();
        loop {
            let mut line = String::new();

            stdout_reader.read_line(&mut line)?;
            dbg!(line.clone());

            if line.contains("\x1a") {
                break;
            }

            stdout.push_str(line.as_str());
        }

        let mut stderr_reader =
            BufReader::new(instance.stderr.as_mut().expect("Failed to open stdout"));

        let mut stderr = String::new();
        loop {
            let mut line = String::new();

            stderr_reader.read_line(&mut line)?;

            dbg!(line.clone());

            if line.contains("\x1a") {
                break;
            }

            stderr.push_str(line.as_str());
        }

        // println!("{}", stdout.trim_end_matches("\n"));

        Ok(CmdOutput::empty())
    }

    fn name(&self) -> String {
        "bash".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
