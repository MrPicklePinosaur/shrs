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
        let mut handle = Command::new("nu").args(vec!["-c", &cmd]).spawn()?;

        handle.wait()?;

        Ok(())
    }
}
