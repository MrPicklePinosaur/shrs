//! The shell builtin that wraps functionality of the History module

// debatable if crate::history should be moved to crate::builtin::history

use std::{
    io::{stdout, Write},
    process::Child,
};

use clap::{Parser, Subcommand};

use crate::shell::{dummy_child, Context, Shell};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Clear,
    Run { index: u32 },
    Search { query: String },
}

pub fn history_builtin(ctx: &mut Context, args: &Vec<String>) -> anyhow::Result<Child> {
    // TODO hack
    let cli = Cli::parse_from(vec!["history".to_string()].iter().chain(args.iter()));

    match &cli.command {
        None => {
            let history = ctx.history.all();
            for (i, h) in history.iter().enumerate() {
                print!("{} {}", i, h);
            }
            stdout().flush()?;
        },
        Some(Commands::Clear) => {
            ctx.history.clear();
        },
        Some(Commands::Run { index }) => {
            todo!()
        },
        Some(Commands::Search { query }) => {
            todo!()
        },
    }

    dummy_child()
}
