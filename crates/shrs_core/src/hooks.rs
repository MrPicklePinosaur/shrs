//! Shell runtime hooks
//!
//! Hooks are user provided functions that are called on a variety of events that occur in the
//! shell. Some additional context is provided to these hooks.
// ideas for hooks
// - on start
// - after prompt
// - before prompt
// - internal error hook (call whenever there is internal shell error; good for debug)
// - env hook (when environment variable is set/changed)
// - exit hook (tricky, make sure we know what cases to call this)

use std::{path::PathBuf, process::ExitStatus, time::Duration};

use crate::{
    cmd_output::CmdOutput,
    shell::{Context, Runtime, Shell},
};

pub type HookFn<C> =
    fn(sh: &Shell, sh_ctx: &mut Context, sh_rt: &mut Runtime, ctx: &C) -> anyhow::Result<()>;

// TODO this is some pretty sus implementation
pub trait Hook<C>: FnMut(&Shell, &mut Context, &mut Runtime, &C) -> anyhow::Result<()> {}

impl<C, T: FnMut(&Shell, &mut Context, &mut Runtime, &C) -> anyhow::Result<()>> Hook<C> for T {}

/// Runs when the shell starts up
#[derive(Clone)]
pub struct StartupCtx {
    /// How long it took the shell to startup
    pub startup_time: Duration,
}

/// Default implementation for [StartupCtx]
pub fn startup_hook(
    _sh: &Shell,
    _sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    _ctx: &StartupCtx,
) -> anyhow::Result<()> {
    println!("welcome to shrs!");
    Ok(())
}

/// Runs before a command is executed
#[derive(Clone)]
pub struct BeforeCommandCtx {
    /// Literal command entered by user
    pub raw_command: String,
    /// Command to be executed, after performing all substitutions
    pub command: String,
    // execution context
    pub run_ctx: Runtime, // TODO a bit heavy to copy the entire run context?
}

/// Default implementation for [BeforeCommandCtx]
pub fn before_command_hook(
    _sh: &Shell,
    _sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    _ctx: &BeforeCommandCtx,
) -> anyhow::Result<()> {
    // let expanded_cmd = format!("[evaluating] {}\n", ctx.command);
    // out.queue(Print(expanded_cmd))?;
    Ok(())
}

/// Runs after a command is executed
#[derive(Clone)]
pub struct AfterCommandCtx {
    /// The command that was ran
    pub command: String,
    /// Command output
    pub cmd_output: CmdOutput,
}

/// Default implementation for [AfterCommandCtx]
pub fn after_command_hook(
    _sh: &Shell,
    _sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    _ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    // let exit_code_str = format!("[exit +{}]\n", ctx.exit_code);
    // out.queue(Print(exit_code_str))?;
    Ok(())
}

/// Runs when a command not found error is received
#[derive(Clone)]
pub struct CommandNotFoundCtx {}

/// Runs when the current working directory is modified
#[derive(Clone)]
pub struct ChangeDirCtx {
    pub old_dir: PathBuf,
    pub new_dir: PathBuf,
}

/// Default implementation for [ChangeDirCtx]
pub fn change_dir_hook(
    _sh: &Shell,
    _sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    _ctx: &ChangeDirCtx,
) -> anyhow::Result<()> {
    Ok(())
}

/// Runs when a job is completed
#[derive(Clone)]
pub struct JobExitCtx {
    pub status: ExitStatus,
}

/// Default implementation for [JobExitCtx]
pub fn job_exit_hook(
    _sh: &Shell,
    _sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    ctx: &JobExitCtx,
) -> anyhow::Result<()> {
    println!("[exit +{:?}]", ctx.status.code());
    Ok(())
}

// /// Hook that runs when a command has a specific exit code
// #[derive(Clone)]
// pub struct ExitStatusCtx<const C: i32> { }

/// Collection of all the hooks that are available
pub struct Hooks {
    // TODO how to uniquely identify a hook? using the Ctx type?
    hooks: anymap::Map,
}

impl Default for Hooks {
    /// Register default hooks
    fn default() -> Self {
        let mut hooks = Hooks::new();

        hooks.insert(startup_hook);
        hooks.insert(before_command_hook);
        hooks.insert(after_command_hook);
        hooks.insert(change_dir_hook);
        hooks.insert(job_exit_hook);

        hooks
    }
}

impl Hooks {
    pub fn new() -> Self {
        Self {
            hooks: anymap::Map::new(),
        }
    }

    /// Registers a new hook
    pub fn insert<C: Clone + 'static>(&mut self, hook: HookFn<C>) {
        match self.hooks.get_mut::<Vec<HookFn<C>>>() {
            Some(hook_list) => {
                hook_list.push(hook);
            },
            None => {
                // register any empty vector for the type
                self.hooks.insert::<Vec<HookFn<C>>>(vec![hook]);
            },
        };
    }

    /*
    /// Register from an iterator
    pub fn register_iter(&mut self) {
        unimplemented!()
    }
    */

    /// Executes all registered hooks
    pub fn run<C: Clone + 'static>(
        &self,
        sh: &Shell,
        sh_ctx: &mut Context,
        sh_rt: &mut Runtime,
        ctx: C,
    ) -> anyhow::Result<()> {
        if let Some(hook_list) = self.hooks.get::<Vec<HookFn<C>>>() {
            for hook in hook_list.iter() {
                (hook)(sh, sh_ctx, sh_rt, &ctx)?;
            }
        }
        Ok(())
    }
}
