//! The most minimal working shell

use std::{process::Command, time::Instant};

use ::anyhow::Result;
use shrs::{
    commands::Commands,
    prelude::*,
    readline::{highlight::IntoHighlighter, line::LineContents},
    state::StateMut,
};
use shrs_utils::styled_buf::StyledBuf;
#[derive(Debug)]
pub struct H {
    i: i32,
}
fn main() {
    let mut hooks = Hooks::new();
    hooks.insert(c);
    hooks.insert(d);
    hooks.insert(e);
    hooks.insert(f);
    hooks.insert_hook(Box::new(Hooo { s: "S".to_string() }));

    let mut bindings = Keybindings::new();
    bindings.insert("C-l", "Clear the screen", |shell: &Shell| -> Result<()> {
        Command::new("clear")
            .spawn()
            .expect("Couldn't clear screen");
        Ok(())
    });
    let myshell = ShellBuilder::default()
        .with_hooks(hooks)
        .with_highlighter(high.into_highlighter())
        .with_keybinding(bindings)
        .with_state(H { i: 10 })
        .build()
        .unwrap();

    myshell.run().expect("Error when running shell");
}

pub fn c(
    mut h: StateMut<H>,
    contents: State<LineContents>,
    sh: &Shell,
    ctx: &AfterCommandCtx,
) -> Result<()> {
    h.i += 1;
    println!("hello");

    Ok(())
}

pub fn d(mut cmd: StateMut<Commands>, h: StateMut<H>, sh: &Shell, ctx: &StartupCtx) -> Result<()> {
    dbg!(h.i);
    cmd.run(|sh: &mut Shell, states: &States| {
        sh.run_hooks(states, SCtx {}).unwrap();
        sh.hooks.insert(g)
    });
    Ok(())
}

pub fn e(sh: &Shell, ctx: &StartupCtx) -> Result<()> {
    dbg!("wqrg");
    Ok(())
}

pub fn f(mut cmd: StateMut<Commands>, sh: &Shell, ctx: &SCtx) -> Result<()> {
    dbg!("wqwe");
    cmd.run(|sh: &mut Shell, states: &States| {
        dbg!("qw");
    });

    Ok(())
}

pub fn g(sh: &Shell, ctx: &AfterCommandCtx) -> Result<()> {
    dbg!("hqwe");
    Ok(())
}

pub fn high(sh: &Shell, buf: &String) -> Result<StyledBuf> {
    Ok(styled_buf!(buf.clone().red()))
}

pub struct Hooo {
    s: String,
}

impl Hook<StartupCtx> for Hooo {
    fn run(&self, sh: &Shell, states: &States, ctx: &StartupCtx) -> Result<()> {
        dbg!(self.s.clone());
        Ok(())
    }
}
#[derive(HookCtx, Clone)]
pub struct SCtx {}
