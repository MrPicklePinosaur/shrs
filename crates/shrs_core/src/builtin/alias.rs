use std::io::{stdout, Write};

use clap::{Parser, Subcommand};

use super::{BuiltinCmd, BuiltinStatus};
use crate::{
    alias::AliasInfo,
    shell::{Context, Runtime, Shell},
};

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
    ) -> anyhow::Result<BuiltinStatus> {
        let cli = Cli::try_parse_from(args)?;

        let mut it = cli.alias.splitn(2, "=");
        let alias_name = it.next().unwrap();
        match it.next() {
            Some(alias_def) => {
                // if alias body is passed, set the alias
                ctx.alias.set(alias_name, AliasInfo::always(alias_def));
            },
            None => {
                // if alias body is not passed, print the alias definition
                /*
                match ctx.alias.get(alias_name) {
                    Some(alias_def) => {
                        println!("alias {}={}", alias_name, alias_def);
                    },
                    None => {
                        eprintln!("{} not defined", alias_name);
                    },
                };
                */
                todo!()
            },
        }

        Ok(BuiltinStatus::success())
    }
}
