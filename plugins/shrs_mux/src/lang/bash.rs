use std::{
    cell::RefCell,
    io::{BufReader, Write},
    process::{Child, Command, Stdio},
};

use shrs::prelude::*;

use crate::interpreter::{read_err, read_out};

pub struct BashLang {
    instance: RefCell<Child>,
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
                    .expect("Failed to start bash process"),
            ),
        }
    }
}

impl Lang for BashLang {
    fn eval(&self, _sh: &Shell, states: &States, cmd: String) -> shrs::anyhow::Result<CmdOutput> {
        let Ok(rt) = states.try_get::<Runtime>() else {
            return Ok(CmdOutput::error());
        };

        let Ok(mut out) = states.try_get_mut::<OutputWriter>() else {
            return Ok(CmdOutput::error());
        };

        let mut instance = self.instance.borrow_mut();
        let stdin = instance.stdin.as_mut().expect("Failed to open stdin");

        let cd_statement = format!("cd {}\n", rt.working_dir.to_string_lossy());

        stdin
            .write_all(cd_statement.as_bytes())
            .expect("unable to set var");

        for (k, v) in rt.env.iter() {
            let export_statement = format!("export {}={:?}\n", k, v);
            stdin
                .write_all(export_statement.as_bytes())
                .expect("unable to set var");
        }
        stdin
            .write_all((cmd + ";echo $?'\x1A'; echo '\x1A' >&2\n").as_bytes())
            .expect("Bash command failed");

        let stdout_reader =
            BufReader::new(instance.stdout.as_mut().expect("Failed to open stdout"));
        let status = read_out(&mut out, stdout_reader)?;

        let stderr_reader =
            BufReader::new(instance.stderr.as_mut().expect("Failed to open stdout"));
        read_err(&mut out, stderr_reader)?;

        Ok(CmdOutput::new(status))
    }

    fn name(&self) -> String {
        "bash".to_string()
    }

    fn needs_line_check(&self, shell: &Shell, ctx: &States) -> bool {
        false
    }
}
