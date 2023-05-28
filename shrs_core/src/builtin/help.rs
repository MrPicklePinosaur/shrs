use std::{
    env,
    path::{Path, PathBuf},
};

use super::{BuiltinCmd, Builtins, Output};
use crate::{
    hooks::ChangeDirCtx,
    shell::{Context, Runtime},
    Shell,
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
    ) -> anyhow::Result<Output> {
        let cmds = sh.builtins.builtins.keys();

        println!("Builtin Commands:");

        for cmd in cmds {
            println!("{}", cmd);
        }

        Ok(Output::success())
    }
}
