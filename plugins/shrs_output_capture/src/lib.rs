//! Capture stdout and stderr of previous command outputs
//!
//!
mod builtin;

use std::{os::unix::process::ExitStatusExt, process::ExitStatus};

use builtin::AgainBuiltin;
use shrs::prelude::*;

struct OutputCaptureState {
    pub last_output: CmdOutput,
}

impl OutputCaptureState {
    pub fn new() -> Self {
        OutputCaptureState {
            last_output: CmdOutput::new("".to_string(), "".to_string(), ExitStatus::from_raw(0)),
        }
    }
}

pub struct OutputCapturePlugin;

impl Plugin for OutputCapturePlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) -> anyhow::Result<()> {
        shell.hooks.register(after_command_hook);
        shell.builtins.insert("again", AgainBuiltin::new());
        shell.state.insert(OutputCaptureState::new());

        Ok(())
    }
}

fn after_command_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<OutputCaptureState>() {
        state.last_output = ctx.cmd_output.clone();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use shrs::{plugin::ShellPlugin, ShellBuilder};

    use crate::OutputCapturePlugin;

    #[test]
    pub fn register() {
        let _myshell = ShellBuilder::default()
            .with_plugin(OutputCapturePlugin)
            .build()
            .unwrap();
    }
}
