use super::BuiltinCmd;
use crate::{
    prelude::{CmdOutput, States},
    shell::{Runtime, Shell},
};

#[derive(Default)]
pub struct ExitBuiltin {}

impl BuiltinCmd for ExitBuiltin {
    fn run(&self, sh: &Shell, _ctx: &mut States, _args: &[String]) -> anyhow::Result<CmdOutput> {
        std::process::exit(0)
    }
}
