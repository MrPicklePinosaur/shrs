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
        let mut out = String::new();

        out += "Builtin Commands:\n";

        for cmd in cmds {
            out += cmd;
            out += "\n";
        }
        println!("{}", out);

        Ok(CmdOutput::stdout(out, 0))
    }
}
