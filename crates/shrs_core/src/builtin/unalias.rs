use clap::{Parser, Subcommand};

use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

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
        _sh: &Shell,
        ctx: &mut Context,
        _rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        if cli.a {
            ctx.alias.clear();
            return Ok(CmdOutput::success());
        }

        for alias in cli.aliases.iter() {
            ctx.alias.unset(alias);
        }

        Ok(CmdOutput::success())
    }
}
