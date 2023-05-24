//! Implementation and runtime for POSIX shell

use std::{
    cell::RefCell,
    collections::HashMap,
    env,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Output, Stdio},
    rc::Rc,
    time::Instant,
};

use anyhow::anyhow;
use crossterm::{style::Print, QueueableCommand};
use lazy_static::lazy_static;
use thiserror::Error;

use crate::{
    alias::Alias,
    builtin::Builtins,
    env::Env,
    hooks::{AfterCommandCtx, BeforeCommandCtx, Hooks, JobExitCtx, StartupCtx},
    jobs::{ExitStatus, Jobs},
    signal::sig_handler,
    state::State,
    theme::Theme,
    Lang,
};

/// Constant shell data
///
/// Data here is generally not mutated at runtime.
pub struct Shell {
    pub hooks: Hooks,
    /// Builtin shell functions that have access to the shell's context
    pub builtins: Builtins,
    /// Color theme
    pub theme: Theme,
    /// The command language
    pub lang: Box<dyn Lang>,
}

/// Shared global shell context
///
/// Context here is shared by each subshell
// TODO can technically unify shell and context
pub struct Context {
    // TODO alias is currently unused
    pub alias: Alias,
    /// Output stream
    pub out: BufWriter<std::io::Stdout>,
    pub state: State,
    pub jobs: Jobs,
    pub startup_time: Instant,
}

/// Runtime context for the shell
///
/// Contains data that can should be local to each subshell. Data here should also be able to be
/// cloned.
#[derive(Clone)]
pub struct Runtime {
    /// Current working directory
    pub working_dir: PathBuf,
    /// Environment variables
    pub env: Env,
    /// Name of the shell or shell script
    pub name: String,
    /// Arguments this shell was called with
    pub args: Vec<String>,
    /// Exit status of most recent pipeline
    pub exit_status: i32,
    // /// List of defined functions
    // pub functions: HashMap<String, Box<ast::Command>>,
}

// some utilitiy commands that should be cleaned up or moved later

pub fn dummy_child() -> anyhow::Result<Child> {
    use std::process::Command;
    let cmd = Command::new("true").spawn()?;
    Ok(cmd)
}

/// Small wrapper that outputs command output if exists
pub fn command_output(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    cmd_handle: &mut Child,
) -> anyhow::Result<ExitStatus> {
    // TODO also handle stderr
    let output = if let Some(out) = cmd_handle.stdout.take() {
        let reader = BufReader::new(out);
        reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                println!("{}", line);
                line
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        String::new()
    };

    // Fetch output status
    let exit_status = cmd_handle.wait().unwrap().code().unwrap();
    rt.exit_status = exit_status;

    // Call hook
    let hook_ctx = AfterCommandCtx {
        exit_code: exit_status,
        cmd_time: 0.0,
        cmd_output: output,
    };
    sh.hooks.run::<AfterCommandCtx>(sh, ctx, rt, hook_ctx)?;

    Ok(ExitStatus(exit_status))
}
