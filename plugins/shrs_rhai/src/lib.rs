mod builtin;
mod rhai;

use rhai::RhaiState;
use shrs::prelude::*;

pub struct RhaiPlugin;

impl Plugin for RhaiPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.builtins.insert("source", builtin::RhaiBuiltin::new());
        // shell.state.insert(RhaiState::new()); // TODO need post-init function that passes shell,
        // rt, and ctx?
        Ok(())
    }
}
