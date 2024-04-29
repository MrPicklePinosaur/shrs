//! Abstraction for the shell language interpreter
//!
//!

mod eval2;
mod posix_lang;
pub use posix_lang::PosixLang;

use crate::{
    cmd_output::CmdOutput,
    prelude::States,
    shell::{Runtime, Shell},
};

/// Trait to implement a shell command language
pub trait Lang {
    fn eval(&self, sh: &Shell, ctx: &States, cmd: String) -> anyhow::Result<CmdOutput>;
    fn name(&self) -> String;
    /// Called when enter is pressed in line to check if the command is complete or needs another
    /// line. Use `state.line.get_full_command()`
    fn needs_line_check(&self, sh: &Shell, ctx: &States) -> bool;
}
