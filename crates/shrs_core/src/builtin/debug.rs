//! Builtin command that has access to shell env for debug and prototyping

use clap::{Parser, Subcommand};

use super::Builtin;
use crate::{
    prelude::{CmdOutput, OutputWriter, States},
    shell::{Runtime, Shell},
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Env,
}

pub fn debug_builtin(sh: &Shell, ctx: &States, args: &[String]) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;

    match &cli.command {
        None => {
            ctx.get_mut::<OutputWriter>().println("debug utility")?;
        },
        Some(Commands::Env) => {
            for (var, val) in ctx.get::<Runtime>().env.iter() {
                let envs = format!("{:?} = {:?}", var, val);
                ctx.get_mut::<OutputWriter>().println(envs)?;
            }
        },
    }

    Ok(CmdOutput::success())
}
