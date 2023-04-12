//! Capture stdout and stderr of previous command outputs
//!
//!

use std::{io::BufWriter, marker::PhantomData};

use shrs::{
    anyhow,
    hooks::AfterCommandCtx,
    plugin::{Plugin, ShellPlugin},
};

pub struct OutputCapturePlugin;

impl Plugin for OutputCapturePlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.hooks.after_command.register(after_command_hook);
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
        let mut myshell = ShellConfigBuilder::default().build().unwrap();

        myshell.with_plugin(OutputCapturePlugin);
    }
}
