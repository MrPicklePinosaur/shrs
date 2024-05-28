use crate::prelude::CmdOutput;

pub fn exit_builtin(_args: &Vec<String>) -> anyhow::Result<CmdOutput> {
    std::process::exit(0)
}
