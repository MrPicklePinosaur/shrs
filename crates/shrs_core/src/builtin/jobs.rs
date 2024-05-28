use crate::prelude::{CmdOutput, Jobs, OutputWriter, StateMut};

pub fn jobs_builtin(
    jobs: StateMut<Jobs>,
    mut out: StateMut<OutputWriter>,
    _args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    for (job_id, _) in jobs.iter() {
        // TODO: This should probably have error handling
        let _ = out.println(job_id.to_string().as_str());
    }

    Ok(CmdOutput::success())
}
