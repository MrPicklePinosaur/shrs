use std::process::{Command, Stdio};

use shrs::prelude::*;

pub struct NuLang {}

impl NuLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for NuLang {
    fn eval(&self, _sh: &Shell, _ctx: &States, cmd: String) -> shrs::anyhow::Result<CmdOutput> {
        let handle = Command::new("nu")
            .args(vec!["-c", &cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        handle.wait_with_output()?;
        // ctx.out.print(output.stdout);

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "nu".to_string()
    }

    fn needs_line_check(&self, shell: &Shell, ctx: &States) -> bool {
        false
    }
}
