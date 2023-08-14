use std::{collections::HashMap, process::Command};

use shrs::prelude::*;

use crate::MuxState;

pub struct MuxLang {
    langs: HashMap<String, Box<dyn Lang>>,
}

impl MuxLang {
    pub fn new(langs: HashMap<String, Box<dyn Lang>>) -> Self {
        // TODO should be configurable later
        Self { langs }
    }
}

impl Lang for MuxLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> anyhow::Result<()> {
        let lang_name = match ctx.state.get::<MuxState>() {
            Some(state) => &state.lang,
            None => return Ok(()),
        };
        // TODO maybe return error if we can't find a lang

        if let Some(lang) = self.langs.get(lang_name) {
            lang.eval(sh, ctx, rt, cmd);
        }

        Ok(())
    }

    fn name(&self) -> String {
        "mux".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}

pub struct NuLang {}

impl NuLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for NuLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<()> {
        let mut handle = Command::new("nu").args(vec!["-c", &cmd]).spawn()?;

        handle.wait()?;

        Ok(())
    }

    fn name(&self) -> String {
        "nu".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}

pub struct PythonLang {}

impl PythonLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for PythonLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<()> {
        let mut handle = Command::new("python").args(vec!["-c", &cmd]).spawn()?;

        handle.wait()?;

        Ok(())
    }

    fn name(&self) -> String {
        "python".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}

pub struct BashLang {}

impl BashLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for BashLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<()> {
        let mut handle = Command::new("bash").args(vec!["-c", &cmd]).spawn()?;

        let exit_status = handle.wait()?;

        // TODO make this generic across all languages later
        let _ = sh.hooks.run(
            sh,
            ctx,
            rt,
            AfterCommandCtx {
                command: cmd.clone(),
                exit_code: exit_status.code().unwrap_or(0), // default to exit code zero
                cmd_output: String::new(),                  // TODO
            },
        );

        Ok(())
    }

    fn name(&self) -> String {
        "bash".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
