use std::{
    env,
    path::{Path, PathBuf},
};

use super::{BuiltinCmd, Builtins};
use crate::{
    hooks::ChangeDirCtx,
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

#[derive(Default)]
pub struct HelpBuiltin {}
impl BuiltinCmd for HelpBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<CmdOutput> {
        let cmds = sh.builtins.builtins.keys();

        ctx.out.println("Builtin Commands")?;

        for cmd in cmds {
            ctx.out.println(cmd)?;
        }

        Ok(CmdOutput::success())
    }
}
