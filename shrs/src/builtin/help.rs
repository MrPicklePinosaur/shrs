use std::{
    env,
    path::{Path, PathBuf},
};

use super::{BuiltinCmd, Builtins};
use crate::{
    hooks::ChangeDirCtx,
    shell::{dummy_child, Context, Runtime},
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
    ) -> anyhow::Result<std::process::Child> {
        let cmds = Builtins::default().builtins.into_keys();

        println!("Builtin Commands:");

        for cmd in cmds {
            println!("{}", cmd);
        }

        dummy_child()
    }
}
