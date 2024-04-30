//! The shell builtin that wraps functionality of the History module

// debatable if crate::history should be moved to crate::builtin::history

use std::io::Cursor;

use clap::{Parser, Subcommand};
use skim::prelude::*;

use super::{Builtin, IntoBuiltin};
use crate::{
    prelude::{CmdOutput, History, OutputWriter, State, StateMut, States},
    prompt_content_queue::{self, PromptContent, PromptContentQueue},
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
pub struct HistoryBuiltin {}
impl Builtin for HistoryBuiltin {
    fn run(&self, sh: &Shell, states: &States, args: &Vec<String>) -> anyhow::Result<CmdOutput> {
        let mut out = states.get_mut::<OutputWriter>();
        let mut prompt_content_queue = states.get_mut::<PromptContentQueue>();
        let cli = Cli::try_parse_from(args)?;

        match &cli.command {
            None => {
                for i in (0..sh.history.len(sh, states)).rev() {
                    out.print(format!(
                        "{}: {}\n",
                        i,
                        sh.history.get(sh, states, i).unwrap()
                    ))?;
                }
            },
            Some(Commands::Clear) => {
                sh.history.clear(sh, states);
            },
            Some(Commands::Run { index }) => {
                if let Some(cmd) = sh.history.get(sh, states, *index as usize) {
                    prompt_content_queue.push(PromptContent::new(cmd.to_string(), true))
                } else {
                    out.print(format!(
                        "Please specificy an index from {} to {} inclusive",
                        0,
                        sh.history.len(sh, states) - 1
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
                for i in 0..sh.history.len(sh, states) {
                    input = format!("{}{}\n", input, sh.history.get(sh, states, i).unwrap());
                }
                let item_reader = SkimItemReader::default();
                let items = item_reader.of_bufread(Cursor::new(input));

                let selected_items = Skim::run_with(&options, Some(items))
                    .map(|out| out.selected_items)
                    .unwrap_or_else(|| Vec::new());

                for item in selected_items.iter() {
                    prompt_content_queue.push(PromptContent::new(item.output().to_string(), true));
                    break;
                }
            },
        }

        Ok(CmdOutput::success())
    }
}
