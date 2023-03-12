use std::{
    io::{stdout, Write},
    process::Child,
};

use clap::{Parser, Subcommand};

use super::BuiltinCmd;
use crate::shell::{dummy_child, Context, Runtime, Shell};

#[derive(Parser)]
struct Cli {
    vars: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {}

#[derive(Default)]
pub struct ExportBuiltin {}

impl BuiltinCmd for ExportBuiltin {
    fn run(
        &self,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<Child> {
        let cli = Cli::parse_from(vec!["export".to_string()].iter().chain(args.iter()));

        for var in cli.vars {
            let mut it = var.splitn(2, "=");
            let var = it.next().unwrap();
            let val = it.next().unwrap_or_default();

            rt.env.set(var, val);
        }

        dummy_child()
    }
}
