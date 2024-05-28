//! Builtin command that has access to shell env for debug and prototyping

use clap::{Parser, Subcommand};

use crate::{
    prelude::{CmdOutput, OutputWriter, State, StateMut},
    shell::Runtime,
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

pub fn debug_builtin(
    rt: State<Runtime>,
    mut out: StateMut<OutputWriter>,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;

    match &cli.command {
        None => {
            out.println("debug utility")?;
        },
        Some(Commands::Env) => {
            for (var, val) in rt.env.iter() {
                let envs = format!("{:?} = {:?}", var, val);
                out.println(envs)?;
            }
        },
    }

    Ok(CmdOutput::success())
}
