use std::{
    io::{stdout, Write},
    process::Child,
};

use clap::{Parser, Subcommand};

use super::BuiltinCmd;
use crate::shell::{dummy_child, Context, Runtime, Shell};

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
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<Child> {
        let cli = Cli::parse_from(vec!["unalias".to_string()].iter().chain(args.iter()));

        if cli.a {
            ctx.alias.clear();
            return dummy_child();
        }

        for alias in cli.aliases.iter() {
            ctx.alias.unset(alias);
        }

        dummy_child()
    }
}
