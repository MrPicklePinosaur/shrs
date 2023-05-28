use std::io::{stdout, Write};

use clap::{Parser, Subcommand};

use super::{BuiltinCmd, Output};
use crate::shell::{Context, Runtime, Shell};

#[derive(Parser)]
struct Cli {
    aliases: Vec<String>,
    #[arg(short)]
    a: bool,
}

#[derive(Subcommand)]
enum Commands {}

#[derive(Default)]
pub struct UnaliasBuiltin {}

impl BuiltinCmd for UnaliasBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<Output> {
        let cli = Cli::parse_from(vec!["unalias".to_string()].iter().chain(args.iter()));

        if cli.a {
            ctx.alias.clear();
            return Ok(Output::success());
        }

        for alias in cli.aliases.iter() {
            ctx.alias.unset(alias);
        }

        Ok(Output::success())
    }
}
