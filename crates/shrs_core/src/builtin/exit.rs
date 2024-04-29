use super::Builtin;
use crate::{
    prelude::{CmdOutput, States},
    shell::{Runtime, Shell},
};

pub fn exit_builtin(sh: &Shell, _args: &Vec<String>) -> anyhow::Result<CmdOutput> {
    std::process::exit(0)
}
