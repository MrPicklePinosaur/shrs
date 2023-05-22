use std::process::Command;

use shrs::prelude::*;

pub struct NuLangPlugin;

impl Plugin for NuLangPlugin {
    fn init(&self, shell: &mut ShellConfig) {
        shell.lang = Box::new(NuLang);
    }
}

pub struct NuLang;

impl Lang for NuLang {
    fn eval(
        &self,
        sh: &shrs::Shell,
        ctx: &mut shrs::Context,
        rt: &mut shrs::Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<()> {
        // TODO kinda dumb cuz the shell has already performed this type of arg splitting already
        let words = cmd
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let mut it = words.iter();

        // Retrieve command name or return immediately (empty command)
        let cmd_name = match it.next() {
            Some(cmd_name) => cmd_name,
            None => return Ok(()),
        };
        let args = it.map(|a| (*a).to_owned().to_string()).collect::<Vec<_>>();

        for (builtin_name, builtin_cmd) in sh.builtins.iter() {
            if builtin_name == cmd_name {
                builtin_cmd.run(sh, ctx, rt, &args)?;
                return Ok(());
            }
        }

        let mut handle = Command::new("nu").args(vec!["-c", &cmd]).spawn()?;

        handle.wait()?;

        Ok(())
    }
}
