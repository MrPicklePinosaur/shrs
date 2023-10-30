use std::{
    collections::HashMap,
    io::Read,
    process::{Command, ExitStatus, Stdio},
};

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
    ) -> anyhow::Result<CmdOutput> {
        let lang_name = match ctx.state.get::<MuxState>() {
            Some(state) => &state.lang,
            None => return Ok(CmdOutput::empty()),
        };
        // TODO maybe return error if we can't find a lang

        if let Some(lang) = self.langs.get(lang_name) {
            return lang.eval(sh, ctx, rt, cmd);
        }

        Ok(CmdOutput::empty())
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
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut handle = Command::new("nu")
            .args(vec!["-c", &cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = handle.wait_with_output()?;

        Ok(CmdOutput::from(output))
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
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut handle = Command::new("python3")
            .args(vec!["-c", &cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = handle.wait_with_output()?;

        Ok(CmdOutput::from(output))
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
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut handle = Command::new("bash")
            .args(vec!["-c", &cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = handle.wait_with_output()?;

        Ok(CmdOutput::from(output))
    }

    fn name(&self) -> String {
        "bash".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
