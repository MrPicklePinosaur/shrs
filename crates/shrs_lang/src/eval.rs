// Lot of code based off of https://github.com/nuta/nsh/blob/main/src/eval.rs

use glob::glob;
use shrs_job::{run_external_command, JobManager, Output, Process, ProcessGroup, Stdin};

use crate::{ast, Lexer, Parser, PosixError};

pub fn eval(job_manager: &mut JobManager, parser: Parser, lexer: Lexer) -> Result<(), PosixError> {

    let parsed = match parser.parse(lexer) {
        Ok(parsed) => parsed,
        Err(e) => {
            // TODO detailed parse errors
            eprintln!("parse error: {e}");
            return Err(PosixError::Parse(e));
        },
    };

    let (procs, pgid) =
        match eval_command(job_manager, &parsed, None, None) {
            Ok((procs, pgid)) => (procs, pgid),
            Err(PosixError::CommandNotFound(_)) => {
                // let _ = cmd.run_hook(CommandNotFoundCtx {});
                // TODO return error code 127
                return Ok(());
            },
            _ => return Ok(()),
        };

    run_job(job_manager, procs, pgid, true)?;
    Ok(())
}

fn run_job(
    job_manager: &mut JobManager,
    procs: Vec<Box<dyn Process>>,
    pgid: Option<u32>,
    foreground: bool,
) -> Result<(), PosixError> {
    let proc_group = ProcessGroup {
        id: pgid,
        processes: procs,
        foreground,
    };

    let job_id = job_manager.create_job("", proc_group);

    if foreground {
        job_manager
            .put_job_in_foreground(Some(job_id), false)
            .map_err(|e| PosixError::Job(e))?;
    } else {
        job_manager
            .put_job_in_background(Some(job_id), false)
            .map_err(|e| PosixError::Job(e))?;
    }
    Ok(())
}

fn expand_arg(arg: &String) -> Vec<String> {
    let mut a = arg.clone();

    // expand ~
    if let Some(remaining) = arg.strip_prefix("~") {
        a = format!(
            "{}{}",
            dirs::home_dir().unwrap().to_string_lossy(),
            remaining
        );
    }

    // quotes escape all special characters
    let first = arg.chars().next().unwrap();
    if first == '\'' || first == '\"' {
        return a
            .trim_matches(|c| c == '\'' || c == '\"')
            .split_whitespace()
            .map(ToString::to_string)
            .collect();
    }
    // match globbed files only if the glob actually works
    else if glob::Pattern::escape(a.as_str()) != a.as_str() {
        if let Ok(files) = glob(a.as_str()) {
            return files
                .filter_map(|file| match file {
                    Ok(s) => Some(s.to_string_lossy().to_string()),
                    Err(s) => Some(s.to_string()),
                })
                .collect();
        }
    }

    vec![a]
}

/// Returns group of processes and also the pgid if it has one
fn eval_command(
    job_manager: &mut JobManager,
    cmd: &ast::Command,
    stdin: Option<Stdin>,
    stdout: Option<Output>,
) -> Result<(Vec<Box<dyn Process>>, Option<u32>), PosixError> {
    match cmd {
        ast::Command::Simple {
            assigns: _,
            redirects: _,
            args,
        } => {
            let mut args_it = args.iter();
            let program = args_it.next().unwrap();
            let args = args_it.flat_map(expand_arg).collect::<Vec<_>>();

            let proc_stdin = stdin.unwrap_or(Stdin::Inherit);
            let proc_stdout = stdout.unwrap_or(Output::Inherit);

            let (proc, pgid) = match run_external_command(
                program,
                &args,
                proc_stdin,
                proc_stdout,
                Output::Inherit,
                None,
            ) {
                Ok((proc, pgid)) => (proc, pgid),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        return Err(PosixError::CommandNotFound(program.clone()))
                    },
                    _ => return Err(PosixError::Eval(e.into())),
                },
            };
            Ok((vec![proc], pgid))
        },
        ast::Command::Pipeline(a_cmd, b_cmd) => {
            let (mut a_procs, _a_pgid) =
                eval_command(job_manager, a_cmd, stdin, Some(Output::CreatePipe))?;
            let (b_procs, b_pgid) = eval_command(
                job_manager,
                b_cmd,
                a_procs.last_mut().unwrap().stdout(),
                stdout,
            )?;
            a_procs.extend(b_procs);
            Ok((a_procs, b_pgid))
        },
        ast::Command::AsyncList(a_cmd, b_cmd) => {
            // TODO double check stdin and stdout
            let (procs, pgid) = eval_command(job_manager, a_cmd, None, None)?;
            run_job(job_manager, procs, pgid, false)?;

            if let Some(b_cmd) = b_cmd {
                eval_command(job_manager, b_cmd, None, None)
            } else {
                Ok((vec![], None))
            }
        },
        ast::Command::None => Ok((vec![], None)),
        _ => todo!(),
    }
}
