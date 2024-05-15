use std::{path::PathBuf, process::ExitStatus, time::Duration};

use shrs_core_macros::HookCtx;

use crate::prelude::{CmdOutput, Runtime, ShellPlugin};

pub trait HookCtx: 'static + std::marker::Send + std::marker::Sync {}

/// Runs when the shell starts up
#[derive(HookCtx)]
pub struct StartupCtx {
    /// How long it took the shell to startup
    pub startup_time: Duration,
}

/// Runs before a command is executed
#[derive(HookCtx)]
pub struct BeforeCommandCtx {
    /// Literal command entered by user
    pub raw_command: String,
    /// Command to be executed, after performing all substitutions
    pub command: String,
}

#[derive(HookCtx)]
pub struct AfterCommandCtx {
    /// The command that was ran
    pub command: String,
    /// Command output
    pub cmd_output: CmdOutput,
}

/// Runs when a command not found error is received
#[derive(HookCtx)]
pub struct CommandNotFoundCtx {}

/// Runs when the current working directory is modified
#[derive(HookCtx)]
pub struct ChangeDirCtx {
    pub old_dir: PathBuf,
    pub new_dir: PathBuf,
}

/// Runs when a job is completed
#[derive(HookCtx)]
pub struct JobExitCtx {
    pub exit_statuses: Vec<ExitStatus>,
}
