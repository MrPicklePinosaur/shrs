// Lot of code based off of https://github.com/nuta/nsh/blob/main/src/eval.rs

use std::process::ExitStatus;

use nix::unistd::{close, pipe, setpgid, Pid};
use shrs_core::{Context, Runtime, Shell};
use shrs_job::{run_external_command, JobManager, Output, Process, ProcessGroup, Stdin};

use crate::ast;

pub struct Os {
    job_manager: JobManager,
    /// Exit status of last command executed.
    last_exit_status: ExitStatus,
}

/// Returns group of processes and also the pgid if it has one
pub fn eval_command(cmd: &ast::Command) -> anyhow::Result<(Vec<Box<dyn Process>>, Option<u32>)> {
    match cmd {
        ast::Command::Simple {
            assigns,
            redirects,
            args,
        } => {
            let mut args_it = args.iter();
            let program = args_it.next().unwrap();
            let args = args_it.collect::<Vec<_>>();
            let (proc, pgid) = run_external_command(
                program,
                &args,
                Stdin::Inherit,
                Output::Inherit,
                Output::Inherit,
                None,
            )?;
            Ok((vec![proc], pgid))
        },
        ast::Command::Pipeline(a_cmd, b_cmd) => {
            // Create a process group
            let (mut a_procs, a_pgid) = eval_command(a_cmd)?;
            let (b_procs, b_pgid) = eval_command(b_cmd)?;
            a_procs.extend(b_procs);
            Ok((a_procs, b_pgid))
        },
        ast::Command::AsyncList(a_cmd, b_cmd) => {
            // create a process group
            todo!()
        },
        ast::Command::None => Ok((vec![], None)),
        _ => todo!(),
    }
}
