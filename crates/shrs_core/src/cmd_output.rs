//! Command output is used by shell builtins as well as shell languages to pass return state of
//! commands or programs. This captures stdout and stderr output, as well as exit code

use std::{os::unix::process::ExitStatusExt, process::ExitStatus};

/// Describes the output of a command
#[derive(Clone, Debug)]
pub struct CmdOutput {
    pub stdout: String,
    pub stderr: String,
    pub status: ExitStatus,
}

impl CmdOutput {
    /// Create a new [CmdOutput] with a specified exit code
    pub fn from_status(status: i32) -> Self {
        CmdOutput {
            stdout: String::new(),
            stderr: String::new(),
            status: ExitStatus::from_raw(status << 8),
        }
    }

    /// Create a new [CmdOutput] with a successful exit code of 0
    pub fn success() -> Self {
        CmdOutput::from_status(0)
    }

    /// Create a new [CmdOutput] with an erroneous exit code of 1
    pub fn error() -> Self {
        CmdOutput::from_status(1)
    }

    // Set the stdout
    pub fn stdout<S: ToString>(&mut self, stdout: S) -> &mut Self {
        self.stdout = stdout.to_string();
        self
    }

    // Set the stderr
    pub fn stderr<S: ToString>(&mut self, stderr: S) -> &mut Self {
        self.stderr = stderr.to_string();
        self
    }
}
