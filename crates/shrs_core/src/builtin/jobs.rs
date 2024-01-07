use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

#[derive(Default)]
pub struct JobsBuiltin {}

impl BuiltinCmd for JobsBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        ctx: &mut Context,
        _rt: &mut Runtime,
        _args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        for (job_id, _) in ctx.jobs.iter() {
            // TODO: This should probably have error handling
            let _ = ctx.out.println(job_id.to_string().as_str());
        }

        Ok(CmdOutput::success())
    }
}
