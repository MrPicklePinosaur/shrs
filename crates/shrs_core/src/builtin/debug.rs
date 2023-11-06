//! Builtin command that has access to shell env for debug and prototyping

use std::io::{stdout, Write};

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
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<CmdOutput> {
        let mut out = String::new();
        let cli = Cli::try_parse_from(args)?;

        match &cli.command {
            None => {
                let s = "debug utility\n";
                print!("{s}");
                out += s;
            },
            Some(Commands::Env) => {
                for (var, val) in rt.env.iter() {
                    let envs = format!("{:?} = {:?}\n", var, val);
                    print!("{envs}");
                    out += envs.as_str();
                }
            },
        }

        Ok(CmdOutput::stdout(out, 0))
    }
}
