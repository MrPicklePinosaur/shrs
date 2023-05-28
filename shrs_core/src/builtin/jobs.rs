use std::{
    env,
    path::{Path, PathBuf},
};

use super::{BuiltinCmd, Output};
use crate::{
    hooks::ChangeDirCtx,
    shell::{Context, Runtime},
    Shell,
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
    ) -> anyhow::Result<Output> {
        for (job_id, _) in ctx.jobs.iter() {
            println!("{}", job_id);
        }

        Ok(Output::success())
    }
}
