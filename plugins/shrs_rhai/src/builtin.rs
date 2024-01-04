use rhai::{Engine, EvalAltResult};
use shrs::prelude::*;

use crate::rhai::RhaiState;

// Run functions defined in rhai script
pub fn after_command_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    // TODO this will make defined functions be shadowed by actual commands, not sure if this is
    // desired behaviour

    // TODO also using invalid command exit status is a bit hacky way of adding extra commands to
    // shell

    // Bash exit code for invalid command
    if let Some(exit_code) = ctx.cmd_output.status.code() {
        if exit_code == 127 {}
    }

    Ok(())
}

#[derive(Default)]
pub struct RhaiBuiltin {}

impl RhaiBuiltin {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCmd for RhaiBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let Some(state) = ctx.state.get_mut::<RhaiState>() else {
            return Ok(CmdOutput::error());
        };

        for file in args.iter().skip(1) {
            let _ = state
                .engine
                .run_file(file.into())
                .map_err(|e| eprintln!("{}", e));
        }

        Ok(CmdOutput::success())
    }
}
