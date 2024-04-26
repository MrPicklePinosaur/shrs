//! The shell builtin that wraps functionality of the History module

// debatable if crate::history should be moved to crate::builtin::history

use std::io::Cursor;

use clap::{Parser, Subcommand};
use skim::prelude::*;

use super::Builtin;
use crate::{
    prelude::{CmdOutput, History, OutputWriter, States},
    prompt_content_queue::{PromptContent, PromptContentQueue},
    shell::{Runtime, Shell},
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

pub fn history_buitin(
    _sh: &Shell,
    states: &mut States,
    args: &[String],
) -> anyhow::Result<CmdOutput> {
    // TODO hack
    let cli = Cli::try_parse_from(args)?;

    match &cli.command {
        None => {
            for i in (0..states.get::<Box<dyn History>>().len()).rev() {
                states.get_mut::<OutputWriter>().print(format!(
                    "{}: {}\n",
                    i,
                    states.get::<Box<dyn History>>().get(i).unwrap()
                ))?;
            }
        },
        Some(Commands::Clear) => {
            states.get_mut::<Box<dyn History>>().clear();
        },
        Some(Commands::Run { index }) => {
            if let Some(cmd) = states.get::<Box<dyn History>>().get(*index as usize) {
                states
                    .get_mut::<PromptContentQueue>()
                    .push(PromptContent::new(cmd.to_string(), true))
            } else {
                states.get_mut::<OutputWriter>().print(format!(
                    "Please specificy an index from {} to {} inclusive",
                    0,
                    states.get::<Box<dyn History>>().len() - 1
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
            for i in 0..states.get::<Box<dyn History>>().len() {
                input = format!(
                    "{}{}\n",
                    input,
                    states.get::<Box<dyn History>>().get(i).unwrap()
                );
            }
            let item_reader = SkimItemReader::default();
            let items = item_reader.of_bufread(Cursor::new(input));

            let selected_items = Skim::run_with(&options, Some(items))
                .map(|out| out.selected_items)
                .unwrap_or_else(|| Vec::new());

            for item in selected_items.iter() {
                states
                    .get_mut::<PromptContentQueue>()
                    .push(PromptContent::new(item.output().to_string(), true));
                break;
            }
        },
    }

    Ok(CmdOutput::success())
}
