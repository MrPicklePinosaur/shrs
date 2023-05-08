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

use std::{io::BufWriter, marker::PhantomData, path::PathBuf};

use crossterm::{style::Print, QueueableCommand};

use crate::{jobs::ExitInfo, Context, Runtime, Shell};

pub type HookFn<C: Clone> =
    fn(sh: &Shell, sh_ctx: &mut Context, sh_rt: &mut Runtime, ctx: &C) -> anyhow::Result<()>;

/// Context for [StartupHook]
#[derive(Clone)]
pub struct StartupCtx {}

/// Default [StartupHook]
pub fn startup_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    _ctx: &StartupCtx,
) -> anyhow::Result<()> {
    println!("welcome to shrs!");
    Ok(())
}

/// Context for [BeforeCommandHook]
#[derive(Clone)]
pub struct BeforeCommandCtx {
    /// Literal command entered by user
    pub raw_command: String,
    /// Command to be executed, after performing all substitutions
    pub command: String,
}
/// Default [BeforeCommandHook]
pub fn before_command_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &BeforeCommandCtx,
) -> anyhow::Result<()> {
    // let expanded_cmd = format!("[evaluating] {}\n", ctx.command);
    // out.queue(Print(expanded_cmd))?;
    Ok(())
}

/// Context for [AfterCommandHook]
#[derive(Clone)]
pub struct AfterCommandCtx {
    /// Exit code of previous command
    pub exit_code: i32,
    /// Amount of time it took to run command
    pub cmd_time: f32,
    /// Command output
    pub cmd_output: String,
}

/// Default [AfterCommandHook]
pub fn after_command_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    // let exit_code_str = format!("[exit +{}]\n", ctx.exit_code);
    // out.queue(Print(exit_code_str))?;
    Ok(())
}

/// Context for [ChangeDirHook]
#[derive(Clone)]
pub struct ChangeDirCtx {
    pub old_dir: PathBuf,
    pub new_dir: PathBuf,
}

/// Default [AfterCommandHook]
pub fn change_dir_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &ChangeDirCtx,
) -> anyhow::Result<()> {
    Ok(())
}

/// Context for [JobExit]
#[derive(Clone)]
pub struct JobExitCtx {
    pub info: ExitInfo,
}

/// Default [JobExitHook]
pub fn job_exit_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &JobExitCtx,
) -> anyhow::Result<()> {
    println!("[exit +{}]", ctx.info.code());
    // println!("{} ms", ctx.info.job_duration.as_millis());
    Ok(())
}

/// Collection of all the hooks that are avaliable
#[derive(Clone)]
pub struct Hooks {
    /// Runs before first prompt is shown
    pub startup: HookList<StartupCtx>,
    /// Runs before each command is executed
    pub before_command: HookList<BeforeCommandCtx>,
    /// Runs after each command is executed
    pub after_command: HookList<AfterCommandCtx>,
    /// Run each time the directory is changed
    pub change_dir: HookList<ChangeDirCtx>,
    /// Run each time the directory is changed
    pub job_exit: HookList<JobExitCtx>,
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
    pub fn run(
        &self,
        sh: &Shell,
        sh_ctx: &mut Context,
        sh_rt: &mut Runtime,
        ctx: &C,
    ) -> anyhow::Result<()> {
        for hook in self.hooks.iter() {
            (hook)(sh, sh_ctx, sh_rt, &ctx)?;
        }
        Ok(())
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
            startup: HookList::from_iter([startup_hook as HookFn<StartupCtx>]),
            before_command: HookList::from_iter([before_command_hook as HookFn<BeforeCommandCtx>]),
            after_command: HookList::from_iter([after_command_hook as HookFn<AfterCommandCtx>]),
            change_dir: HookList::from_iter([change_dir_hook as HookFn<ChangeDirCtx>]),
            job_exit: HookList::from_iter([job_exit_hook as HookFn<JobExitCtx>]),
        }
    }
}
