//! Hook definitions that are emitted by the shell

use std::{path::PathBuf, process::ExitStatus, time::Duration};

use crate::prelude::{CmdOutput, HookEvent, HookEventMarker};

/// Runs when the shell starts up
#[derive(HookEvent)]
pub struct StartupCtx {
    /// How long it took the shell to startup
    pub startup_time: Duration,
}

/// Runs before a command is executed
#[derive(HookEvent)]
pub struct BeforeCommandCtx {
    /// Literal command entered by user
    pub raw_command: String,
    /// Command to be executed, after performing all substitutions
    pub command: String,
}

/// Runs after a command has completed
#[derive(HookEvent)]
pub struct AfterCommandCtx {
    /// The command that was ran
    pub command: String,
    /// Command output
    pub cmd_output: CmdOutput,
}

/// Runs when a command not found error is received
#[derive(HookEvent)]
pub struct CommandNotFoundCtx {}

/// Runs when the current working directory is modified
#[derive(HookEvent)]
pub struct ChangeDirCtx {
    pub old_dir: PathBuf,
    pub new_dir: PathBuf,
}

/// Runs when a job is completed
///
/// Multiple jobs may have completed at the same time so a vector of exit statuses is returned
#[derive(HookEvent)]
pub struct JobExitCtx {
    pub exit_statuses: Vec<ExitStatus>,
}
