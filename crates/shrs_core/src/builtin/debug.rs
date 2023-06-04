//! Builtin command that has access to shell env for debug and prototyping

use std::io::{stdout, Write};

use clap::{Parser, Subcommand};

use super::{BuiltinCmd, BuiltinStatus};
use crate::shell::{Context, Runtime, Shell};

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
    ) -> anyhow::Result<BuiltinStatus> {
        let cli = Cli::parse_from(vec!["debug".to_string()].iter().chain(args.iter()));

        match &cli.command {
            None => {
                println!("debug utility");
            },
            Some(Commands::Env) => {
                for (var, val) in rt.env.iter() {
                    println!("{:?} = {:?}", var, val);
                }
            },
        }

        Ok(BuiltinStatus::success())
    }
}
