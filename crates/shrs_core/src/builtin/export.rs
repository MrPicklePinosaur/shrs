use clap::{Parser, Subcommand};

use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

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
        _sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        // remove arg
        if cli.n {
            for var in cli.vars {
                rt.env.remove(&var)?;
            }
            return Ok(CmdOutput::success());
        }

        // print all env vars
        if cli.p {
            for (var, val) in rt.env.iter() {
                let s = format!("export {:?}={:?}", var, val);
                ctx.out.println(s)?;
            }
            return Ok(CmdOutput::success());
        }

        for var in cli.vars {
            let mut it = var.splitn(2, '=');
            let var = it.next().unwrap();
            let val = it.next().unwrap_or_default();

            rt.env.set(var, val)?;
        }

        Ok(CmdOutput::success())
    }
}
