use std::io::{stdout, Write};

use clap::{Parser, Subcommand};

use super::{BuiltinCmd, Output};
use crate::shell::{Context, Runtime, Shell};

#[derive(Parser)]
struct Cli {
    alias: String,
}

#[derive(Subcommand)]
enum Commands {}

#[derive(Default)]
pub struct AliasBuiltin {}

impl BuiltinCmd for AliasBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<Output> {
        let cli = match Cli::try_parse_from(vec!["alias".to_string()].iter().chain(args.iter())) {
            Ok(cli) => cli,
            Err(e) => {
                eprintln!("{}", e);
                return Ok(Output::error());
            },
        };

        let mut it = cli.alias.splitn(2, "=");
        let alias_name = it.next().unwrap();
        match it.next() {
            Some(alias_def) => {
                // if alias body is passed, set the alias
                ctx.alias.set(alias_name, alias_def);
            },
            None => {
                // if alias body is not passed, print the alias definition
                match ctx.alias.get(alias_name) {
                    Some(alias_def) => {
                        println!("alias {}={}", alias_name, alias_def);
                    },
                    None => {
                        eprintln!("{} not defined", alias_name);
                    },
                };
            },
        }

        Ok(Output::success())
    }
}
