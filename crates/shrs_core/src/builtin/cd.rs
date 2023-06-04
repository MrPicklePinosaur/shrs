use std::{
    env,
    path::{Path, PathBuf},
};

use super::{BuiltinCmd, BuiltinStatus};
use crate::{
    hooks::ChangeDirCtx,
    shell::{Context, Runtime, Shell},
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

        let old_pwd = env::current_dir().unwrap();
        rt.env.set("OLDPWD", &old_pwd.display().to_string());

        env::set_current_dir(path.clone())?; // TODO should env current dir remain as the directory the shell was started in?

        let hook_ctx = ChangeDirCtx {
            old_dir: old_pwd,
            new_dir: path.clone(),
        };
        // need to be able to call hook from here
        sh.hooks.run::<ChangeDirCtx>(sh, ctx, rt, hook_ctx);

        rt.working_dir = path.clone();
        rt.env.set("PWD", path.to_str().unwrap());

        // return a dummy command
        Ok(BuiltinStatus::success())
    }
}
