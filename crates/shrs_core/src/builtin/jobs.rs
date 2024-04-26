use super::BuiltinCmd;
use crate::{
    prelude::{CmdOutput, Jobs, OutputWriter, States},
    shell::{Runtime, Shell},
};

#[derive(Default)]
pub struct JobsBuiltin {}

impl BuiltinCmd for JobsBuiltin {
    fn run(&self, sh: &Shell, ctx: &mut States, _args: &[String]) -> anyhow::Result<CmdOutput> {
        for (job_id, _) in ctx.get_mut::<Jobs>().iter() {
            // TODO: This should probably have error handling
            let _ = ctx
                .get_mut::<OutputWriter>()
                .println(job_id.to_string().as_str());
        }

        Ok(CmdOutput::success())
    }
}
