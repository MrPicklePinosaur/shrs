use std::path::{Path, PathBuf};

use clap::Parser;

use crate::{
    prelude::{CmdOutput, OutputWriter},
    shell::{set_working_dir, Runtime, Shell},
    state::StateMut,
};

#[derive(Parser)]
struct Cli {
    path: Option<String>,
}

pub fn cd_builtin(
    mut rt: StateMut<Runtime>,
    mut out: StateMut<OutputWriter>,
    sh: &Shell,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;
    let path = if let Some(path) = cli.path {
        // `cd -` moves us back to previous directory
        if path == "-" {
            if let Ok(old_pwd) = rt.env.get("OLDPWD") {
                PathBuf::from(old_pwd)
            } else {
                out.eprintln("no OLDPWD")?;
                return Ok(CmdOutput::error());
            }
        } else if let Some(remaining) = path.strip_prefix("~") {
            match dirs::home_dir() {
                Some(home) => PathBuf::from(format!("{}{}", home.to_string_lossy(), remaining)),
                None => {
                    out.eprintln("No Home Directory")?;
                    return Ok(CmdOutput::error());
                },
            }
        } else {
            rt.working_dir.join(Path::new(&path))
        }
    } else {
        dirs::home_dir().unwrap()
    };

    if let Err(e) = set_working_dir(sh, &mut rt, &path, true) {
        out.eprintln(e)?;
        return Ok(CmdOutput::error());
    }

    // return a dummy command
    Ok(CmdOutput::success())
}
