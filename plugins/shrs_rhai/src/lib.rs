mod builtin;
mod rhai;

use builtin::after_command_hook;
use rhai::RhaiState;
use shrs::prelude::*;

pub struct RhaiPlugin;

impl Plugin for RhaiPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.builtins.insert("source", builtin::RhaiBuiltin::new());
        shell.hooks.insert(after_command_hook);
        Ok(())
    }

    fn post_init(&self, sh: &Shell, ctx: &mut Context, rt: &mut Runtime) -> anyhow::Result<()> {
        let state = RhaiState::new(sh, ctx, rt);
        ctx.state.insert(state);
        Ok(())
    }
}
