//! sh.rs - a rusty shell library
//!
//!

#[macro_use]
extern crate derive_builder;

// TODO refactor to
// mod <mod>
// pub use <mod>::{ ... }

mod alias;
pub use alias::Alias;

pub mod builtin;

mod env;
pub use env::Env;

pub mod hooks;

pub mod prompt;

mod shell;
pub use shell::{
    dummy_child, find_executables_in_path, Context, Runtime, Shell, ShellConfig,
    ShellConfigBuilder, ShellConfigBuilderError,
};
pub use shrs_line as line;

mod signal;

pub mod theme;

pub mod plugin;

mod state;

// TODO temp re-export anyhow
pub use anyhow;

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
