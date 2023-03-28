use super::BuiltinCmd;
use crate::shell::{Context, Runtime};

#[derive(Default)]
pub struct ExitBuiltin {}

impl BuiltinCmd for ExitBuiltin {
    fn run(
        &self,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<std::process::Child> {
        std::process::exit(0)
    }
}
