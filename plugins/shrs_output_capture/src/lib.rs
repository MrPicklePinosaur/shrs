//! Capture stdout and stderr of previous command outputs
//!
//!
mod builtin;

use builtin::AgainBuiltin;
use shrs::{anyhow, hooks::AfterCommandCtx, plugin::Plugin, Context, Runtime, Shell};

struct OutputCaptureState {
    pub last_command: String,
}

impl OutputCaptureState {
    pub fn new() -> Self {
        OutputCaptureState {
            last_command: String::new(),
        }
    }
}

pub struct OutputCapturePlugin;

impl Plugin for OutputCapturePlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.hooks.after_command.register(after_command_hook);
        shell.builtins.insert("again", AgainBuiltin::new());
        shell.state.insert(OutputCaptureState::new());
    }
}

fn after_command_hook(
    _sh: &Shell,
    sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<OutputCaptureState>() {
        state.last_command = ctx.cmd_output.clone();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use shrs::{plugin::ShellPlugin, ShellConfigBuilder};

    use crate::OutputCapturePlugin;

    #[test]
    pub fn register() {
        let _myshell = ShellConfigBuilder::default()
            .with_plugin(OutputCapturePlugin)
            .build()
            .unwrap();
    }
}
