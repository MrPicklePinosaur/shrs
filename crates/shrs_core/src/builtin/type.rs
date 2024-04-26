use std::fs::metadata;

use clap::Parser;

use super::BuiltinCmd;
use crate::{
    prelude::{Alias, CmdOutput, OutputWriter, Runtime, States},
    shell::Shell,
};

#[derive(Parser)]
struct Cli {
    #[arg(short = 'P')]
    path_search_only: bool,
    #[arg(short = 'p')]
    path_result_only: bool,
    #[arg(short)]
    type_only: bool,
    #[arg(short)]
    all: bool,
    names: Vec<String>,
}

#[derive(Default)]
pub struct TypeBuiltin {}

fn analyze_name(
    name: &String,
    path_search_only: bool,
    path_result_only: bool,
    type_only: bool,
    all: bool,
    sh: &Shell,
    states: &mut States,
) -> anyhow::Result<CmdOutput> {
    let mut name_found = false;
    if !path_search_only {
        // check if name is an alias
        let alias = states.get::<Alias>();
        let alias_matches = alias.get_subst(name);
        if alias_matches.is_some() {
            name_found = true;
            if !path_result_only {
                if type_only {
                    states.get_mut::<OutputWriter>().println("alias")?;
                } else {
                    states.get_mut::<OutputWriter>().println(format!(
                        "{} is aliased to `{}'",
                        name,
                        alias_matches.unwrap()
                    ))?;
                }
            }
            if !all {
                return Ok(CmdOutput::success());
            }
        }

        // check if name is a builtin
        if sh.builtins.builtins.contains_key(name as &str) {
            name_found = true;
            if !path_result_only {
                if type_only {
                    states.get_mut::<OutputWriter>().println("builtin")?;
                } else {
                    states
                        .get_mut::<OutputWriter>()
                        .println(format!("{} is a shell builtin", name))?;
                }
            }
            if !all {
                return Ok(CmdOutput::success());
            }
        }
    }

    // check if name is in path
    for dir in states.get::<Runtime>().env.get("PATH")?.split(":") {
        let full_path = format!("{}/{}", dir, name);
        let md = metadata(&full_path);
        if md.is_ok() && md.unwrap().is_file() {
            if type_only {
                states.get_mut::<OutputWriter>().println("file")?;
            } else if path_search_only {
                states.get_mut::<OutputWriter>().println(&full_path)?;
            } else {
                states
                    .get_mut::<OutputWriter>()
                    .println(format!("{} is {}", name, &full_path))?;
            }
            return Ok(CmdOutput::success());
        }
    }

    if name_found {
        return Ok(CmdOutput::success());
    }

    states
        .get_mut::<OutputWriter>()
        .eprintln(format!("-shrs: type: {} not found", name))?;
    Ok(CmdOutput::success())
}

impl BuiltinCmd for TypeBuiltin {
    fn run(&self, sh: &Shell, ctx: &mut States, args: &[String]) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        let success = cli
            .names
            .iter()
            .map(|n| {
                analyze_name(
                    n,
                    cli.path_search_only,
                    cli.path_result_only,
                    cli.type_only,
                    cli.all,
                    sh,
                    ctx,
                )
            })
            .all(|r| r.is_ok());

        if success {
            Ok(CmdOutput::success())
        } else {
            Ok(CmdOutput::error())
        }
    }
}
