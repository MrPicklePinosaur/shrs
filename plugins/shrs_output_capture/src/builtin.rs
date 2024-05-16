use shrs::prelude::*;

use crate::OutputCaptureState;

pub fn again_builtin(
    state: State<OutputCaptureState>,
    _sh: &Shell,
    _args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    print!("{}", state.last_output.stdout);
    print!("{}", state.last_output.stderr);

    Ok(CmdOutput::success())
}
