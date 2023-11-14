use std::{
    env,
    path::{Path, PathBuf},
};

use super::BuiltinCmd;
use crate::{
    hooks::ChangeDirCtx,
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

#[derive(Default)]
pub struct JobsBuiltin {}

impl BuiltinCmd for JobsBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<CmdOutput> {
        for (job_id, _) in ctx.jobs.iter() {
            ctx.out.println(job_id.to_string().as_str());
        }

        Ok(CmdOutput::success())
    }
}
