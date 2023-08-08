use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use super::{BuiltinCmd, BuiltinStatus};
use crate::shell::{Context, Runtime, Shell};

lazy_static! {
    static ref SHEBANG_REGEX: Regex = Regex::new(r"#!(?P<interp>.+)").unwrap();
}

#[derive(Parser)]
struct Cli {
    source_file: String,
}

#[derive(Default)]
pub struct SourceBuiltin {}

impl BuiltinCmd for SourceBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<BuiltinStatus> {
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
                println!("using interp {} at {}", interp.as_str(), &cli.source_file);
                let mut child = Command::new(interp.as_str())
                    .args(vec![cli.source_file])
                    .spawn()?;

                // need command output here
                // TODO temp disable this
                // command_output(sh, ctx, rt, &mut child)?;

                Ok(BuiltinStatus::success())
            },
            None => {
                // otherwise evaluate with self

                todo!()
            },
        }
    }
}
