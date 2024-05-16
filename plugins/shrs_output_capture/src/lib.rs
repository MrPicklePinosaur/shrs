//! Capture stdout and stderr of previous command outputs
//!
//!
mod builtin;

use builtin::again_builtin;
use shrs::prelude::*;

pub struct OutputCaptureState {
    pub last_command: String,
    pub last_output: CmdOutput,
}

impl OutputCaptureState {
    pub fn new() -> Self {
        OutputCaptureState {
            last_command: String::new(),
            last_output: CmdOutput::success(),
        }
    }
}

pub struct OutputCapturePlugin;

impl Plugin for OutputCapturePlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.hooks.insert(after_command_hook);
        shell.builtins.insert("again", again_builtin);
        shell.states.insert(OutputCaptureState::new());

        Ok(())
    }
    fn meta(&self) -> PluginMeta {
        PluginMeta::new(
            "Output Capture",
            "Plugin that stores the last command that was ran and the last output",
            None,
        )
    }
}

fn after_command_hook(
    mut state: StateMut<OutputCaptureState>,
    _sh: &Shell,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    state.last_command = ctx.command.clone();
    state.last_output = ctx.cmd_output.clone();
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::{OutputCapturePlugin, ShellBuilder};

    #[test]
    pub fn register() {
        let _myshell = ShellBuilder::default()
            .with_plugin(OutputCapturePlugin)
            .build()
            .unwrap();
    }
}
