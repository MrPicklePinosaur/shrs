//! Abstraction for the shell language interpreter
//!
//!

use crate::shell::{Context, Runtime, Shell};

/// Trait to implement a shell command langauge
pub trait Lang {
    // TODO make function signature of this MUCH more generic
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> anyhow::Result<()>;
}
