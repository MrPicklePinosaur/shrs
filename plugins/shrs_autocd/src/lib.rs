use std::fs;

use shrs::{line::_core::shell::set_working_dir, prelude::*};

pub struct AutocdPlugin;

pub fn after_command_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    // Bash exit code for invalid command
    if let Some(exit_code) = ctx.cmd_output.status.code() {
        if exit_code == 127 {
            // Check if the command name matches a directory
            let Some(cmd_name) = ctx.command.split(' ').next() else {
                return Ok(());
            };

            let paths = fs::read_dir("./").unwrap();

            for path in paths {
                let path = path.unwrap();
                println!("{:?}", path);
                if path.file_type().unwrap().is_dir() && path.file_name() == cmd_name {
                    set_working_dir(sh, sh_ctx, sh_rt, &path.path(), true)?;
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

impl Plugin for AutocdPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.hooks.register(after_command_hook);

        Ok(())
    }
}
