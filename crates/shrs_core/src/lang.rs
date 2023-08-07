//! Abstraction for the shell language interpreter
//!
//!

use crate::shell::{Context, Runtime, Shell};

/// Trait to implement a shell command language
pub trait Lang {
    // TODO make function signature of this MUCH more generic
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> anyhow::Result<()>;
    fn name(&self) -> String;
    fn needs_line_check(&self, cmd: String) -> bool;
}
