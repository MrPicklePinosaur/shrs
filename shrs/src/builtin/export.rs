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
    #[arg(short)]
    p: bool,
    #[arg(short)]
    n: bool,
}

#[derive(Subcommand)]
enum Commands {}

#[derive(Default)]
pub struct ExportBuiltin {}

impl BuiltinCmd for ExportBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<Child> {
        let cli = Cli::parse_from(vec!["export".to_string()].iter().chain(args.iter()));

        // remove arg
        if cli.n {
            for var in cli.vars {
                rt.env.remove(&var);
            }
            return dummy_child();
        }

        // print all env vars
        if cli.p {
            for (var, val) in rt.env.all().iter() {
                println!("export {}={}", var, val);
            }
            return dummy_child();
        }

        for var in cli.vars {
            let mut it = var.splitn(2, "=");
            let var = it.next().unwrap();
            let val = it.next().unwrap_or_default();

            rt.env.set(var, val);
        }

        dummy_child()
    }
}
