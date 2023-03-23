//! sh.rs - a rusty shell library

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate derive_builder;

lalrpop_mod!(pub grammar);

// TODO refactor to
// mod <mod>
// pub use <mod>::{ ... }

pub mod alias;
mod ast;
pub mod builtin;
pub mod env;
pub mod hooks;
mod lexer;
mod parser;
pub mod prompt;
pub mod shell;
mod signal;

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
