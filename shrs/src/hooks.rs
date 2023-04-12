//! Shell runtime hooks
//!
//! Hooks are user provided functions that are called on a variety of events that occur in the
//! shell. Some additional context is provided to these hooks.
// ideas for hooks
// - on start
// - after prompt
// - before prompt
// - internal error hook (call whenever there is internal shell error; good for debug)
// - env hook (when envrionment variable is set/changed)
// - exit hook (tricky, make sure we know what cases to call this)

use std::{io::BufWriter, marker::PhantomData};

use crossterm::{style::Print, QueueableCommand};

pub type HookFn<C> = fn(out: &mut BufWriter<std::io::Stdout>, ctx: C) -> anyhow::Result<()>;

/// Context for [StartupHook]
pub struct StartupCtx {
    /// How much time it has taken for the shell to initialize
    pub startup_time: usize,
}

/// Default [StartupHook]
pub fn startup_hook(out: &mut BufWriter<std::io::Stdout>, _ctx: StartupCtx) -> anyhow::Result<()> {
    println!("welcome to shrs!");
    Ok(())
}

/// Context for [BeforeCommandHook]
pub struct BeforeCommandCtx {
    /// Literal command entered by user
    pub raw_command: String,
    /// Command to be executed, after performing all substitutions
    pub command: String,
}
/// Default [BeforeCommandHook]
pub fn before_command_hook(
    out: &mut BufWriter<std::io::Stdout>,
    ctx: BeforeCommandCtx,
) -> anyhow::Result<()> {
    // let expanded_cmd = format!("[evaluating] {}\n", ctx.command);
    // out.queue(Print(expanded_cmd))?;
    Ok(())
}

/// Context for [AfterCommandHook]
pub struct AfterCommandCtx {
    /// Exit code of previous command
    pub exit_code: i32,
    /// Amount of time it took to run command
    pub cmd_time: f32,
}

/// Default [AfterCommandHook]
pub fn after_command_hook(
    out: &mut BufWriter<std::io::Stdout>,
    ctx: AfterCommandCtx,
) -> anyhow::Result<()> {
    // let exit_code_str = format!("[exit +{}]\n", ctx.exit_code);
    // out.queue(Print(exit_code_str))?;
    Ok(())
}

/// Collection of all the hooks that are avaliable
#[derive(Clone)]
pub struct Hooks {
    /// Runs before first prompt is shown
    pub startup: HookFn<StartupCtx>,
    /// Runs before each command is executed
    pub before_command: HookFn<BeforeCommandCtx>,
    /// Runs after each command is executed
    pub after_command: HookFn<AfterCommandCtx>,
}

#[derive(Clone)]
pub struct HookList<C> {
    hooks: Vec<HookFn<C>>,
}

impl<C> HookList<C> {
    pub fn new() -> Self {
        HookList { hooks: vec![] }
    }

    /// Registers a new hook
    pub fn register(&mut self, hook: HookFn<C>) {
        self.hooks.push(hook);
    }

    /// Executes all registered hooks
    pub fn run(&self, out: &mut BufWriter<std::io::Stdout>, ctx: C) {
        for hook in self.hooks {
            (hook)(out, ctx);
        }
    }
}

impl<C> Default for HookList<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> FromIterator<HookFn<C>> for HookList<C> {
    fn from_iter<T: IntoIterator<Item = HookFn<C>>>(iter: T) -> Self {
        HookList {
            hooks: Vec::from_iter(iter),
        }
    }
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
