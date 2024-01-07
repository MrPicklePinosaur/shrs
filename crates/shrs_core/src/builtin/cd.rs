use std::path::{Path, PathBuf};

use clap::Parser;

use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    shell::{set_working_dir, Context, Runtime, Shell},
};

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

#[derive(Default)]
pub struct CdBuiltin {}

impl BuiltinCmd for CdBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        let path = if let Some(path) = cli.path {
            // `cd -` moves us back to previous directory
            if path == "-" {
                if let Ok(old_pwd) = rt.env.get("OLDPWD") {
                    PathBuf::from(old_pwd)
                } else {
                    ctx.out.eprintln("no OLDPWD")?;
                    return Ok(CmdOutput::error());
                }
            } else {
                rt.working_dir.join(Path::new(&path))
            }
        } else {
            let home_dir = rt.env.get("HOME").unwrap();
            Path::new(&home_dir).to_path_buf()
        };

        if let Err(e) = set_working_dir(sh, ctx, rt, &path, true) {
            ctx.out.eprintln(e)?;
            return Ok(CmdOutput::error());
        }

        // return a dummy command
        Ok(CmdOutput::success())
    }
}
