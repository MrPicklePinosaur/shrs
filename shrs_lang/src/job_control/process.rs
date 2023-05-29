use std::{
    fmt, iter,
    process::{Child, ChildStdout, Command, ExitStatus, Stdio},
};

use super::io::Stdin;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProcessId(u32);

impl From<u32> for ProcessId {
    fn from(value: u32) -> Self {
        ProcessId(value)
    }
}

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

struct BuiltinProcess {
    argv: Vec<String>,
    status_code: ExitStatus,
    stdout: Option<Stdin>,
}

impl BuiltinProcess {
    pub fn new<S1, S2>(
        program: S1,
        args: &[S2],
        status_code: ExitStatus,
        stdout: Option<Stdin>,
    ) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            argv: iter::once(program)
                .map(|p| p.as_ref().to_string())
                .chain(args.iter().map(|arg| arg.as_ref().to_string()))
                .collect(),
            status_code,
            stdout,
        }
    }
}

impl Process for BuiltinProcess {
    fn id(&self) -> Option<ProcessId> {
        None
    }

    fn argv(&self) -> String {
        self.argv[..].join(" ")
    }

    fn status(&self) -> ProcessStatus {
        ProcessStatus::Completed
    }

    fn status_code(&self) -> Option<ExitStatus> {
        Some(self.status_code)
    }

    fn stdout(&mut self) -> Option<Stdin> {
        self.stdout.take()
    }

    fn kill(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn wait(&mut self) -> anyhow::Result<ExitStatus> {
        Ok(self.status_code)
    }

    fn try_wait(&mut self) -> anyhow::Result<Option<ExitStatus>> {
        Ok(Some(self.status_code))
    }
}

struct ExternalProcess {
    argv: Vec<String>,
    child: Child,
    status: ProcessStatus,
    status_code: Option<ExitStatus>,
}

impl ExternalProcess {
    pub fn new<S1, S2>(program: S1, args: &[S2], child: Child) -> Self
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            argv: iter::once(&program)
                .map(|p| p.as_ref().to_string())
                .chain(args.iter().map(|arg| arg.as_ref().to_string()))
                .collect(),
            child,
            status: ProcessStatus::Running,
            status_code: None,
        }
    }
}

impl Process for ExternalProcess {
    fn id(&self) -> Option<ProcessId> {
        Some(self.child.id().into())
    }

    fn argv(&self) -> String {
        self.argv[..].join(" ")
    }

    fn status(&self) -> ProcessStatus {
        self.status
    }

    fn status_code(&self) -> Option<ExitStatus> {
        self.status_code
    }

    fn stdout(&mut self) -> Option<Stdin> {
        self.child.stdout.take().map(Stdin::Child)
    }

    fn kill(&mut self) -> anyhow::Result<()> {
        self.child.kill()?;
        Ok(())
    }

    fn wait(&mut self) -> anyhow::Result<ExitStatus> {
        let exit_status = self.child.wait()?;
        self.status = ProcessStatus::Completed;
        self.status_code = Some(exit_status);
        Ok(exit_status)
    }

    fn try_wait(&mut self) -> anyhow::Result<Option<ExitStatus>> {
        if let Some(exit_status) = self.child.try_wait()? {
            self.status = ProcessStatus::Completed;
            self.status_code = Some(exit_status);
            Ok(Some(exit_status))
        } else {
            Ok(None)
        }
    }
}
