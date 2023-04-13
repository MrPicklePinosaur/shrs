//! Capture stdout and stderr of previous command outputs
//!
//!
mod builtin;

use std::{io::BufWriter, marker::PhantomData};

use builtin::AgainBuiltin;
use shrs::{
    anyhow,
    hooks::AfterCommandCtx,
    plugin::{Plugin, ShellPlugin},
};

pub struct OutputCapturePlugin;

impl Plugin for OutputCapturePlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.hooks.after_command.register(after_command_hook);
        shell.builtins.insert("again", AgainBuiltin::new());
    }
}

fn after_command_hook(
    out: &mut BufWriter<std::io::Stdout>,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    println!("Output Capture Hook registered");
    Ok(())
}

#[cfg(test)]
mod tests {
    use shrs::{plugin::ShellPlugin, ShellConfigBuilder};

    use crate::OutputCapturePlugin;

    #[test]
    pub fn register() {
        let myshell = ShellConfigBuilder::default()
            .with_plugin(OutputCapturePlugin)
            .build()
            .unwrap();
    }
}
