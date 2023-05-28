use super::{BuiltinCmd, Output};
use crate::{
    shell::{Context, Runtime},
    Shell,
};

#[derive(Default)]
pub struct ExitBuiltin {}

impl BuiltinCmd for ExitBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<Output> {
        std::process::exit(0)
    }
}
