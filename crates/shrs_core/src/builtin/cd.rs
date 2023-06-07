use std::{
    env,
    path::{Path, PathBuf},
};

use super::{BuiltinCmd, BuiltinStatus};
use crate::{
    hooks::ChangeDirCtx,
    shell::{set_working_dir, Context, Runtime, Shell},
};

#[derive(Default)]
pub struct CdBuiltin {}

impl BuiltinCmd for CdBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<BuiltinStatus> {
        let path = if let Some(path) = args.get(0) {
            // `cd -` moves us back to previous directory
            if path == "-" {
                if let Ok(old_pwd) = rt.env.get("OLDPWD") {
                    PathBuf::from(old_pwd)
                } else {
                    eprintln!("no OLDPWD");
                    return Ok(BuiltinStatus::error());
                }
            } else {
                rt.working_dir.join(Path::new(path))
            }
        } else {
            let home_dir = rt.env.get("HOME").unwrap();
            Path::new(&home_dir).to_path_buf()
        };

        if let Err(_) = set_working_dir(sh, ctx, rt, &path, true) {
            return Ok(BuiltinStatus::error());
        }

        // return a dummy command
        Ok(BuiltinStatus::success())
    }
}
