// Lot of code based off of https://github.com/nuta/nsh/blob/main/src/eval.rs

use std::process::ExitStatus;

use nix::unistd::{close, pipe, setpgid, Pid};
use shrs_core::{Context, Runtime, Shell};

use crate::{ast, job_control::JobManager};

pub struct Os {
    job_manager: JobManager,
    /// Exit status of last command executed.
    last_exit_status: ExitStatus,
}

pub fn eval_command(cmd: &ast::Command) -> anyhow::Result<ExitStatus> {
    match cmd {
        ast::Command::Simple {
            assigns,
            redirects,
            args,
        } => {
            todo!()
        },
        ast::Command::Pipeline(pipeline) => {
            todo!()
        },
        ast::Command::AsyncList(a_cmd, b_cmd) => {
            todo!()
        },
        _ => todo!(),
    }
}
