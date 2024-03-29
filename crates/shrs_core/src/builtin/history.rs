//! The shell builtin that wraps functionality of the History module

// debatable if crate::history should be moved to crate::builtin::history

use std::io::Cursor;

use clap::{Parser, Subcommand};
use skim::prelude::*;

use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    prompt_content_queue::PromptContent,
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
                    _ctx.out
                        .print(format!("{}: {}\n", i, _ctx.history.get(i).unwrap()))?;
                }
            },
            Some(Commands::Clear) => {
                _ctx.history.clear();
            },
            Some(Commands::Run { index }) => {
                if let Some(cmd) = _ctx.history.get(*index as usize) {
                    _ctx.prompt_content_queue
                        .push(PromptContent::new(cmd.to_string(), true))
                } else {
                    _ctx.out.print(format!(
                        "Please specificy an index from {} to {} inclusive",
                        0,
                        _ctx.history.len() - 1
                    ))?;
                }
            },
            Some(Commands::Search { query }) => {
                // We expect Skim to succeed
                let options = SkimOptionsBuilder::default()
                    .height(Some("100%"))
                    .nosort(true)
                    .query(Some(query))
                    .build()
                    .unwrap();

                let mut input = String::new();
                for i in 0.._ctx.history.len() {
                    input = format!("{}{}\n", input, _ctx.history.get(i).unwrap());
                }
                let item_reader = SkimItemReader::default();
                let items = item_reader.of_bufread(Cursor::new(input));

                let selected_items = Skim::run_with(&options, Some(items))
                    .map(|out| out.selected_items)
                    .unwrap_or_else(|| Vec::new());

                for item in selected_items.iter() {
                    _ctx.prompt_content_queue
                        .push(PromptContent::new(item.output().to_string(), true));
                    break;
                }
            },
        }

        Ok(CmdOutput::success())
    }
}
