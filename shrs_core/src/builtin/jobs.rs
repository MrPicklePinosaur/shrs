use std::{
    borrow::Borrow,
    env,
    path::{Path, PathBuf},
};

use super::{BuiltinCmd, BuiltinStatus};
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
    ) -> anyhow::Result<BuiltinStatus> {
        let job_manager = sh.job_manager.borrow();
        for job in job_manager.get_jobs() {
            println!("{} {}", job.id(), job.display());
        }

        Ok(BuiltinStatus::success())
    }
}
