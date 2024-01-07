//! Core functionality of shrs
extern crate derive_builder;
extern crate lazy_static;

pub mod alias;
pub mod builtin;
pub mod cmd_output;
pub mod env;
pub mod history;
pub mod hooks;
pub mod jobs;
pub mod keybinding;
pub mod lang;
pub mod output_writer;
pub mod prompt;
pub mod shell;
pub mod signal;
pub mod state;
pub mod theme;

pub mod prelude {
    //! Conveniently import commonly used types

    pub use crate::{
        alias::{Alias, AliasInfo, AliasRule, AliasRuleCtx},
        builtin::{BuiltinCmd, Builtins},
        cmd_output::CmdOutput,
        env::Env,
        hooks::{Hook, HookFn, Hooks, *},
        jobs::{JobId, JobInfo, Jobs},
        keybinding::{parse_keybinding, BindingFn, DefaultKeybinding, Keybinding},
        lang::Lang,
        output_writer::OutputWriter,
        prompt::*,
        shell::{Context, Runtime, Shell},
        signal::Signals,
        state::State,
        theme::Theme,
    };
}

/*
#[cfg(test)]
mod tests {
    use rexpect::session::PtySession;

    fn spawn_proc() -> anyhow::Result<PtySession> {
        let p = rexpect::spawn("cargo run --example tester", Some(2000))?;
        Ok(p)
    }

    #[test]
    fn echo() -> anyhow::Result<()> {
        let mut p = spawn_proc()?;

        p.send_line("echo hi")?;
        p.exp_regex("hi")?;

        p.send_control('c')?;
        Ok(())
    }

    #[test]
    fn pipes() -> anyhow::Result<()> {
        let mut p = spawn_proc()?;

        p.send_line("echo hello | tr e o")?;
        p.exp_regex("hollo")?;

        p.send_line("echo hello | tr e o | tr o a")?;
        p.exp_regex("halla")?;

        p.send_control('c')?;
        Ok(())
    }
}
*/
