use std::{
    env,
    path::{Path, PathBuf},
};

use super::BuiltinCmd;
use crate::{
    hooks::ChangeDirCtx,
    shell::{dummy_child, Context, Runtime},
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
    ) -> anyhow::Result<std::process::Child> {
        for (job_id, _) in ctx.jobs.iter() {
            println!("{}", job_id);
        }

        dummy_child()
    }
}
