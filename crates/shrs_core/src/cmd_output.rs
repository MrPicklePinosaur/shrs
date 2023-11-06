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
        CmdOutput::new(String::new(), String::new(), 0)
    }
    pub fn new(stdout: String, stderr: String, status: i32) -> Self {
        CmdOutput {
            stdout,
            stderr,
            status: ExitStatus::from_raw(status),
        }
    }
    pub fn stdout(stdout: String, status: i32) -> Self {
        CmdOutput::new(stdout, String::new(), status)
    }
    pub fn stderr(stderr: String, status: i32) -> Self {
        CmdOutput::new(String::new(), stderr, status)
    }
    pub fn success() -> Self {
        CmdOutput::new(String::new(), String::new(), 0)
    }
    pub fn error() -> Self {
        CmdOutput::new(String::new(), String::new(), 1)
    }
}
impl From<process::Output> for CmdOutput {
    fn from(o: process::Output) -> Self {
        CmdOutput {
            stdout: String::from_utf8_lossy(&o.stdout).to_string(),
            stderr: String::from_utf8_lossy(&o.stderr).to_string(),
            status: o.status,
        }
    }
}
