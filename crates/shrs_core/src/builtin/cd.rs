use std::path::{Path, PathBuf};

use clap::Parser;

use super::Builtin;
use crate::{
    prelude::{CmdOutput, OutputWriter, States},
    shell::{set_working_dir, Runtime, Shell},
};

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

pub fn cd_builtin(sh: &Shell, ctx: &mut States, args: &[String]) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;
    let path = if let Some(path) = cli.path {
        // `cd -` moves us back to previous directory
        if path == "-" {
            if let Ok(old_pwd) = ctx.get::<Runtime>().env.get("OLDPWD") {
                PathBuf::from(old_pwd)
            } else {
                ctx.get_mut::<OutputWriter>().eprintln("no OLDPWD")?;
                return Ok(CmdOutput::error());
            }
        } else if let Some(remaining) = path.strip_prefix("~") {
            match dirs::home_dir() {
                Some(home) => PathBuf::from(format!("{}{}", home.to_string_lossy(), remaining)),
                None => {
                    ctx.get_mut::<OutputWriter>()
                        .eprintln("No Home Directory")?;
                    return Ok(CmdOutput::error());
                },
            }
        } else {
            ctx.get::<Runtime>().working_dir.join(Path::new(&path))
        }
    } else {
        dirs::home_dir().unwrap()
    };

    if let Err(e) = set_working_dir(sh, ctx, &path, true) {
        ctx.get_mut::<OutputWriter>().eprintln(e)?;
        return Ok(CmdOutput::error());
    }

    // return a dummy command
    Ok(CmdOutput::success())
}
