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
    ) -> anyhow::Result<CmdOutput> {
        if let Some(state) = ctx.state.get::<OutputCaptureState>() {
            print!("{}", state.last_output.stdout);
            print!("{}", state.last_output.stderr);
        }

        Ok(CmdOutput::success())
    }
}
