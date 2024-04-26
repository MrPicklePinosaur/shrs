//! The most minimal working shell

use std::time::Instant;

use ::anyhow::Result;
use shrs::{commands::Commands, prelude::*, state::StateMut};
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
    let myshell = ShellBuilder::default()
        .with_hooks(hooks)
        .with_state(H { i: 10 })
        .build()
        .unwrap();

    myshell.run().expect("Error when running shell");
}
pub fn c(mut h: StateMut<H>, sh: &Shell, ctx: &StartupCtx) -> Result<()> {
    h.i += 1;

    Ok(())
}
pub fn d(mut cmd: StateMut<Commands>, h: StateMut<H>, sh: &Shell, ctx: &StartupCtx) -> Result<()> {
    dbg!(h.i);
    cmd.add(|sh: &mut Shell, states: &States| {
        sh.run_hooks(states, SCtx {}).unwrap();
        sh.hooks.insert(g)
    });
    Ok(())
}
pub fn e(sh: &Shell, ctx: &StartupCtx) -> Result<()> {
    dbg!("wqrg");
    Ok(())
}
pub fn f(sh: &Shell, ctx: &SCtx) -> Result<()> {
    dbg!("wqwe");
    Ok(())
}
pub fn g(sh: &Shell, ctx: &AfterCommandCtx) -> Result<()> {
    dbg!("hqwe");
    Ok(())
}
pub struct SCtx {}
impl Ctx for SCtx {}
