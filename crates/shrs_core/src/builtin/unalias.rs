use clap::{Parser, Subcommand};

use super::Builtin;
use crate::{
    prelude::{Alias, CmdOutput, States},
    shell::{Runtime, Shell},
};

#[derive(Parser)]
struct Cli {
    aliases: Vec<String>,
    #[arg(short)]
    a: bool,
}

#[derive(Subcommand)]
enum Commands {}

pub fn unalias_builtin(sh: &Shell, ctx: &mut States, args: &[String]) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;

    if cli.a {
        ctx.get_mut::<Alias>().clear();
        return Ok(CmdOutput::success());
    }

    for alias in cli.aliases.iter() {
        ctx.get_mut::<Alias>().unset(alias);
    }

    Ok(CmdOutput::success())
}
