use shrs_core::{Context, Runtime, Shell};

use crate::{
    ast, process,
    process::{run_process, ExitStatus, Pgid},
};

pub fn eval_command(cmd: &ast::Command, ctx: &process::Context) -> anyhow::Result<ExitStatus> {
    match cmd {
        ast::Command::Simple {
            assigns,
            redirects,
            args,
        } => {
            run_process(args, Pgid::Current, ctx)?;
            Ok(ExitStatus::Exited(0))
        },
        _ => todo!(),
    }
}
