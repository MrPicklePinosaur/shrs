use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

#[derive(Default)]
pub struct ExitBuiltin {}

impl BuiltinCmd for ExitBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        _ctx: &mut Context,
        _rt: &mut Runtime,
        _args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        std::process::exit(0)
    }
}
