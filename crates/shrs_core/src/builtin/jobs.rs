use super::Builtin;
use crate::{
    prelude::{CmdOutput, Jobs, OutputWriter, States},
    shell::{Runtime, Shell},
};

pub fn jobs_builtin(sh: &Shell, ctx: &mut States, _args: &[String]) -> anyhow::Result<CmdOutput> {
    for (job_id, _) in ctx.get_mut::<Jobs>().iter() {
        // TODO: This should probably have error handling
        let _ = ctx
            .get_mut::<OutputWriter>()
            .println(job_id.to_string().as_str());
    }

    Ok(CmdOutput::success())
}
