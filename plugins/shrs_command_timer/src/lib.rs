//! Capture stdout and stderr of previous command outputs
//!
//!

use std::{
    io::BufWriter,
    marker::PhantomData,
    time::{Duration, Instant},
};

use shrs::{
    anyhow,
    hooks::{AfterCommandCtx, BeforeCommandCtx},
    plugin::{Plugin, ShellPlugin},
    Context, Runtime, Shell,
};

pub struct CommandTimerState {
    /// The time the previous command was started at
    start_time: Option<Instant>,
    /// Buffer to hold the result of the previous tracked command time
    prev_command_time: Option<Duration>,
}

impl CommandTimerState {
    pub fn new() -> Self {
        Self {
            start_time: None,
            prev_command_time: None,
        }
    }

    /// Start the timer
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// End the timer and reset it
    ///
    /// No-op if start was never called
    pub fn end(&mut self) {
        if let Some(start_time) = self.start_time {
            self.prev_command_time = Some(Instant::now().duration_since(start_time));
        }
        self.start_time = None;
    }

    /// Fetch the previous command time
    pub fn command_time(&self) -> Option<Duration> {
        self.prev_command_time
    }
}

pub struct CommandTimerPlugin;

impl Plugin for CommandTimerPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.hooks.before_command.register(before_command_hook);
        shell.hooks.after_command.register(after_command_hook);
        shell.state.insert(CommandTimerState::new());
    }
}

fn before_command_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &BeforeCommandCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<CommandTimerState>() {
        state.start();
    }
    Ok(())
}

fn after_command_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    if let Some(state) = sh_ctx.state.get_mut::<CommandTimerState>() {
        state.end()
    }
    Ok(())
}
