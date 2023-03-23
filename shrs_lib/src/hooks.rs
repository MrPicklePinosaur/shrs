// ideas for hooks
// - on start
// - after prompt
// - before prompt
// - internal error hook (call whenever there is internal shell error; good for debug)
// - env hook (when envrionment variable is set/changed)
// - exit hook (tricky, make sure we know what cases to call this)

use std::io::BufWriter;

use crossterm::{style::Print, QueueableCommand};

pub struct StartupHookCtx {
    pub startup_time: usize,
}

pub type StartupHook = fn(ctx: StartupHookCtx);
pub fn startup_hook(_ctx: StartupHookCtx) {
    println!("welcome to shrs!");
}

pub struct BeforeCommandCtx {
    /// Literal command entered by user
    pub raw_command: String,
    /// Command to be executed, after performing all substitutions
    pub command: String,
}
pub type BeforeCommandHook =
    fn(out: &mut BufWriter<std::io::Stdout>, ctx: BeforeCommandCtx) -> anyhow::Result<()>;

pub fn before_command_hook(
    out: &mut BufWriter<std::io::Stdout>,
    ctx: BeforeCommandCtx,
) -> anyhow::Result<()> {
    // let expanded_cmd = format!("[evaluating] {}\n", ctx.command);
    // out.queue(Print(expanded_cmd))?;
    Ok(())
}

pub struct AfterCommandCtx {
    /// Exit code of previous command
    pub exit_code: i32,
    /// Amount of time it took to run command
    pub cmd_time: f32,
}
pub type AfterCommandHook =
    fn(out: &mut BufWriter<std::io::Stdout>, ctx: AfterCommandCtx) -> anyhow::Result<()>;

pub fn after_command_hook(
    out: &mut BufWriter<std::io::Stdout>,
    ctx: AfterCommandCtx,
) -> anyhow::Result<()> {
    // let exit_code_str = format!("[exit +{}]\n", ctx.exit_code);
    // out.queue(Print(exit_code_str))?;
    Ok(())
}

#[derive(Clone)]
pub struct Hooks {
    /// Runs before first prompt is shown
    pub startup: StartupHook,
    pub before_command: BeforeCommandHook,
    pub after_command: AfterCommandHook,
}

impl Default for Hooks {
    fn default() -> Self {
        Hooks {
            startup: startup_hook,
            before_command: before_command_hook,
            after_command: after_command_hook,
        }
    }
}
