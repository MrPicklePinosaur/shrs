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

pub fn run_job(
    job_manager: &mut JobManager,
    procs: Vec<Box<dyn Process>>,
    pgid: Option<u32>,
    foreground: bool,
) -> anyhow::Result<()> {
    if procs.is_empty() {
        return Ok(());
    }

    let proc_group = ProcessGroup {
        id: pgid,
        processes: procs,
        foreground,
    };

    let is_foreground = proc_group.foreground;
    let job_id = job_manager.create_job("", proc_group);

    if is_foreground {
        job_manager.put_job_in_foreground(Some(job_id), false)?;
    } else {
        job_manager.put_job_in_background(Some(job_id), false)?;
    }
    Ok(())
}

/// Returns group of processes and also the pgid if it has one
pub fn eval_command(
    job_manager: &mut JobManager,
    cmd: &ast::Command,
    pgid: Option<u32>,
    stdin: Option<Stdin>,
    stdout: Option<Output>,
) -> anyhow::Result<(Vec<Box<dyn Process>>, Option<u32>)> {
    match cmd {
        ast::Command::Simple {
            assigns,
            redirects,
            args,
        } => {
            let mut args_it = args.iter();
            let program = args_it.next().unwrap();
            let args = args_it.collect::<Vec<_>>();

            let proc_stdin = stdin.unwrap_or(Stdin::Inherit);
            let proc_stdout = stdout.unwrap_or(Output::Inherit);

            let (proc, pgid) = run_external_command(
                program,
                &args,
                proc_stdin,
                proc_stdout,
                Output::Inherit,
                pgid,
            )?;
            Ok((vec![proc], pgid))
        },
        ast::Command::Pipeline(a_cmd, b_cmd) => {
            let (mut a_procs, pgid) =
                eval_command(job_manager, a_cmd, pgid, stdin, Some(Output::CreatePipe))?;
            let (b_procs, pgid) = eval_command(
                job_manager,
                b_cmd,
                pgid,
                a_procs.last_mut().unwrap().stdout(),
                stdout,
            )?;
            a_procs.extend(b_procs);
            Ok((a_procs, pgid))
        },
        ast::Command::AsyncList(a_cmd, b_cmd) => {
            // TODO double check stdin and stdout
            let (procs, pgid) = eval_command(job_manager, a_cmd, pgid, None, None)?;
            run_job(job_manager, procs, pgid, false)?;

            if let Some(b_cmd) = b_cmd {
                eval_command(job_manager, b_cmd, pgid, None, None)
            } else {
                Ok((vec![], None))
            }
        },
        ast::Command::None => Ok((vec![], None)),
        _ => todo!(),
    }
}
