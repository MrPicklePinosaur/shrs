use std::{fs::read_to_string, path::PathBuf, process::Command};

use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use super::Builtin;
use crate::{
    prelude::{CmdOutput, OutputWriter, StateMut, States},
    shell::{Runtime, Shell},
};

lazy_static! {
    static ref SHEBANG_REGEX: Regex = Regex::new(r"#!(?P<interp>.+)").unwrap();
}

#[derive(Parser)]
struct Cli {
    source_file: String,
}

pub fn source_builtin(
    mut out: StateMut<OutputWriter>,
    sh: &Shell,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;

    let file_path = PathBuf::from(&cli.source_file);
    let file_contents = read_to_string(file_path)?;

    // read shebang from first line
    let mut it = file_contents.lines();

    let interp = it
        .next()
        .and_then(|first_line| SHEBANG_REGEX.captures(first_line))
        .and_then(|capture| capture.name("interp"));

    match interp {
        Some(interp) => {
            let s = format!("using interp {} at {}", interp.as_str(), &cli.source_file);
            out.println(s)?;
            let mut _child = Command::new(interp.as_str())
                .args(vec![cli.source_file])
                .spawn()?;

            // need command output here
            // TODO temp disable this
            // command_output(sh, ctx, rt, &mut child)?;

            Ok(CmdOutput::success())
        },
        None => {
            // otherwise evaluate with self

            todo!()
        },
    }
}
