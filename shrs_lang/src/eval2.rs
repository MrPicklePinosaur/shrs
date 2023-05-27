// Lot of code based off of https://github.com/nuta/nsh/blob/main/src/eval.rs

use nix::unistd::{close, pipe, setpgid, Pid};
use shrs_core::{Context, Runtime, Shell};

use crate::{
    ast,
    process::{self, run_process, ExitStatus, Job, Os, Pgid, ProcessState},
};

pub fn eval_command(
    os: &mut Os,
    cmd: &ast::Command,
    ctx: &process::Context,
) -> anyhow::Result<ExitStatus> {
    match cmd {
        ast::Command::Simple {
            assigns,
            redirects,
            args,
        } => {
            run_process(args, Pgid::Current, ctx)?;
            Ok(ExitStatus::Exited(0))
        },
        ast::Command::Pipeline(pipeline) => {
            let mut pipeline_iter = pipeline.iter().peekable();
            let mut pgid: Option<Pid> = None;
            let mut res: Option<ExitStatus> = None; // Exit status of last command in pipeline
            let mut cur_stdin = ctx.stdin;
            let mut children: Vec<Pid> = vec![]; // Keep track of all the child processes in this pipeline

            while let Some(pipe_cmd) = pipeline_iter.next() {
                // Construct pipe as required
                let cur_stdout;
                let pipe = if pipeline_iter.peek().is_some() {
                    let (pipe_out, pipe_in) = pipe()?;
                    cur_stdout = pipe_in;
                    Some((pipe_out, pipe_in))
                } else {
                    // if last item in pipeline output directly to passed in stdout
                    cur_stdout = ctx.stdout;
                    None
                };

                let ctx = process::Context {
                    stdin: cur_stdin,
                    stdout: cur_stdout,
                    ..*ctx
                };
                let a_res = eval_command(os, pipe_cmd, &ctx)?;

                // Close pipe if necessary
                if let Some((pipe_out, pipe_in)) = pipe {
                    cur_stdin = pipe_out;
                    close(pipe_in)?;
                }

                res = match a_res {
                    ExitStatus::Exited(status) => Some(ExitStatus::Exited(status)),
                    ExitStatus::Running(pid) => {
                        // Set pgid if is leader in pipeline
                        if pgid.is_none() {
                            pgid = Some(pid);
                        }

                        if ctx.is_interactive {
                            setpgid(pid, pgid.unwrap())?;
                        }

                        children.push(pid);
                        Some(ExitStatus::Running(pid))
                    },
                };
            }

            let res = match res {
                Some(ExitStatus::Exited(status)) => ExitStatus::Exited(status),
                Some(ExitStatus::Running(pid)) => {
                    // wait for last command in pipeline to finish
                    let jobid = os.create_job(pid, children)?;

                    // Wait for job to finish executing and report exit status
                    match os.wait_for_job(jobid)? {
                        ProcessState::Running => ExitStatus::Running(pgid.unwrap()),
                        ProcessState::Exited(status) => ExitStatus::Exited(status),
                    }
                },
                None => ExitStatus::Exited(0),
            };
            Ok(res)
        },
        _ => todo!(),
    }
}
