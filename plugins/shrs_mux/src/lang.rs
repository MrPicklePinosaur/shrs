use std::process::Command;

use shrs::prelude::*;

pub struct NuLang;

impl Lang for NuLang {
    fn eval(
        &self,
        sh: &shrs::Shell,
        ctx: &mut shrs::Context,
        rt: &mut shrs::Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<()> {
        let mut words_it = cmd
            .split(' ')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        // Retrieve command name or return immediately (empty command)
        let cmd_name = match words_it.next() {
            Some(cmd_name) => cmd_name,
            None => return Ok(()),
        };
        let args = words_it
            .map(|s| s.to_owned().to_string())
            .collect::<Vec<_>>();

        for (builtin_name, builtin_cmd) in sh.builtins.iter() {
            if builtin_name == &cmd_name {
                builtin_cmd.run(sh, ctx, rt, &args)?;
                continue;
            }
        }

        let mut handle = Command::new("nu").args(vec!["-c", &cmd]).spawn()?;

        handle.wait()?;

        Ok(())
    }
}
