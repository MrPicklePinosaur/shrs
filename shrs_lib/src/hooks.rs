// ideas for hooks
// - on start
// - after prompt
// - before prompt
// - internal error hook (call whenever there is internal shell error; good for debug)
// - env hook (when envrionment variable is set/changed)
// - exit hook (tricky, make sure we know what cases to call this)

pub struct StartupHookCtx {
    pub startup_time: usize,
}

pub type StartupHook = fn(ctx: StartupHookCtx);
fn _startup_hook(_ctx: StartupHookCtx) {
    println!("welcome to shrs!");
}

pub type ExitCodeHook = fn(code: i32);
fn _exit_code_hook(code: i32) {
    println!("[exit +{}]", code);
}

#[derive(Clone)]
pub struct Hooks {
    /// Runs before first prompt is shown
    pub startup: StartupHook,
    /// Formatter for displaying the exit code of the previous command
    pub exit_code: ExitCodeHook,
}

impl Default for Hooks {
    fn default() -> Self {
        Hooks {
            startup: _startup_hook,
            exit_code: _exit_code_hook,
        }
    }
}
