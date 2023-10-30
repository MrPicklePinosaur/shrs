use shrs::prelude::*;

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
    ) -> anyhow::Result<BuiltinStatus> {
        if let Some(state) = ctx.state.get::<OutputCaptureState>() {
            print!("stdout: {}", state.last_output.stdout);
            print!("stderr: {}", state.last_output.stderr);
        }

        Ok(BuiltinStatus::success())
    }
}
