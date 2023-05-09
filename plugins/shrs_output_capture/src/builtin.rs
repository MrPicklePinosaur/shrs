use shrs::{anyhow, builtin::BuiltinCmd, dummy_child, Context, Runtime, Shell};

use crate::OutputCaptureState;

#[derive(Default)]
pub struct AgainBuiltin {}

impl AgainBuiltin {
    pub fn new() -> Self {
        AgainBuiltin {}
    }
}

impl BuiltinCmd for AgainBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        ctx: &mut Context,
        _rt: &mut Runtime,
        _args: &Vec<String>,
    ) -> anyhow::Result<std::process::Child> {
        if let Some(state) = ctx.state.get::<OutputCaptureState>() {
            print!("{}", state.last_command);
        }

        dummy_child()
    }
}
