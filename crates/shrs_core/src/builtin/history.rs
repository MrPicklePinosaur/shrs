//! The shell builtin that wraps functionality of the History module

// debatable if crate::history should be moved to crate::builtin::history

use clap::{Parser, Subcommand};

use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

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

#[derive(Default)]
pub struct HistoryBuiltin {}

impl BuiltinCmd for HistoryBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        _ctx: &mut Context,
        _rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        // TODO hack
        let cli = Cli::try_parse_from(args)?;

        match &cli.command {
            None => {
                for i in (0.._ctx.history.len()).rev() {   
                    _ctx.out.print(format!("{}: {}\n", i, _ctx.history.get(i).unwrap()))?;
                }
            },  
            Some(Commands::Clear) => {
                _ctx.history.clear();
            },
            Some(Commands::Run { index }) => {
                let index = *index as usize;
                if (0.._ctx.history.len()).contains(&index) {
                    //Run _ctx.history.get(index).unwrap()
                }
            },
            Some(Commands::Search { .. }) => {
                todo!()
            },
        }

        Ok(CmdOutput::success())
    }
}
