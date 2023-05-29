use std::{
    fmt,
    process::{Child, ChildStdout, Command, ExitStatus, Stdio},
};

use super::io::Stdin;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProcessId(u32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProcessStatus {
    Running,
    Stopped,
    Completed,
}

pub trait Process {
    fn id(&self) -> Option<ProcessId>;
    fn argv(&self) -> String;
    fn status(&self) -> ProcessStatus;
    fn status_code(&self) -> Option<ExitStatus>;
    fn stdout(&mut self) -> Option<Stdin>;
    fn kill(&mut self) -> anyhow::Result<()>;
    fn wait(&mut self) -> anyhow::Result<ExitStatus>;
    fn try_wait(&mut self) -> anyhow::Result<Option<ExitStatus>>;
}

impl fmt::Debug for dyn Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Process {{ id: {} }}",
            self.id()
                .map(|id| id.0.to_string())
                .unwrap_or_else(|| "(builtin)".to_string())
        )
    }
}

#[derive(Debug)]
pub struct ProcessGroup {
    pub id: Option<u32>,
    pub processes: Vec<Box<dyn Process>>,
    pub foreground: bool,
}
