use super::Builtin;
use crate::{
    prelude::{CmdOutput, Jobs, OutputWriter, StateMut, States},
    shell::{Runtime, Shell},
};

pub fn jobs_builtin(
    mut jobs: StateMut<Jobs>,
    mut out: StateMut<OutputWriter>,
    sh: &Shell,
    _args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    for (job_id, _) in jobs.iter() {
        // TODO: This should probably have error handling
        let _ = out.println(job_id.to_string().as_str());
    }

    Ok(CmdOutput::success())
}
