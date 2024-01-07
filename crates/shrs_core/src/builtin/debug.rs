//! Builtin command that has access to shell env for debug and prototyping

use clap::{Parser, Subcommand};

use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
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

#[derive(Default)]
pub struct DebugBuiltin {}

impl BuiltinCmd for DebugBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        match &cli.command {
            None => {
                ctx.out.println("debug utility")?;
            },
            Some(Commands::Env) => {
                for (var, val) in rt.env.iter() {
                    let envs = format!("{:?} = {:?}", var, val);
                    ctx.out.println(envs)?;
                }
            },
        }

        Ok(CmdOutput::success())
    }
}
