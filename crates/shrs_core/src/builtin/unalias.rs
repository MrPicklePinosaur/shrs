use clap::{Parser, Subcommand};

use crate::prelude::{Alias, CmdOutput, StateMut};

#[derive(Parser)]
struct Cli {
    aliases: Vec<String>,
    #[arg(short)]
    a: bool,
}

#[derive(Subcommand)]
enum Commands {}

pub fn unalias_builtin(
    mut aliases: StateMut<Alias>,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;

    if cli.a {
        aliases.clear();
        return Ok(CmdOutput::success());
    }

    for alias in cli.aliases.iter() {
        aliases.unset(alias);
    }

    Ok(CmdOutput::success())
}
