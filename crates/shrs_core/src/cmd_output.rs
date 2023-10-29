use std::{
    os::unix::process::ExitStatusExt,
    process::{self, ExitStatus, Output},
};

use pino_deref::Deref;

#[derive(Clone, Debug)]
pub struct CmdOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}
impl CmdOutput {
    pub fn empty() -> CmdOutput {
        CmdOutput::new(String::new(), String::new(), ExitStatus::from_raw(0))
    }
    pub fn new(stdout: String, stderr: String, status: ExitStatus) -> Self {
        CmdOutput {
            stdout,
            stderr,
            status,
        }
    }
    pub fn from_process_output(o: process::Output) -> Self {
        CmdOutput {
            stdout: String::from_utf8_lossy(&o.stdout).to_string(),
            stderr: String::from_utf8_lossy(&o.stderr).to_string(),
            status: o.status,
        }
    }
    //if stdout or stderr are empty this will only return one, otherwise err after out
    pub fn out(&self) -> String {
        self.stdout.clone() + self.stderr.as_str()
    }
}
