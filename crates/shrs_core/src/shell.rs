//! Types for internal context of shell

use std::{
    cell::RefCell,
    env,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::anyhow;
use log::error;
use shrs_job::JobManager;

use crate::{
    alias::Alias,
    builtin::Builtins,
    env::Env,
    history::History,
    hooks::{ChangeDirCtx, Hooks},
    jobs::Jobs,
    keybinding::Keybinding,
    lang::Lang,
    output_writer::OutputWriter,
    signal::Signals,
    state::State,
    theme::Theme,
};

/// Constant shell data
///
/// Data here is generally not mutated at runtime.
pub struct Shell {
    pub job_manager: RefCell<JobManager>,
    pub hooks: Hooks,
    /// Builtin shell functions that have access to the shell's context
    pub builtins: Builtins,
    /// Color theme
    pub theme: Theme,
    /// The command language
    pub lang: Box<dyn Lang>,
    /// Signals to be handled
    pub signals: Signals,
    pub keybinding: Box<dyn Keybinding>,
}

/// Shared global shell context
///
/// Context here is shared by each subshell
// TODO can technically unify shell and context
pub struct Context {
    /// Output stream
    pub out: OutputWriter,
    pub state: State,
    pub jobs: Jobs,
    pub startup_time: Instant,
    pub alias: Alias,
    pub history: Box<dyn History<HistoryItem = String>>,
}

/// Runtime context for the shell
///
/// Contains data that can should be local to each subshell. Data here should also be able to be
/// cloned.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct Runtime {
    /// Current working directory
    pub working_dir: PathBuf,
    /// Environment variables
    pub env: Env,
    /// Name of the shell or shell script
    pub name: String,
    /// Arguments this shell was called with
    pub args: Vec<String>,
    /// Exit status of most recent pipeline
    pub exit_status: i32,
    // /// List of defined functions
    // pub functions: HashMap<String, Box<ast::Command>>,
}

/// Set the current working directory
pub fn set_working_dir(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    wd: &Path,
    run_hook: bool,
) -> anyhow::Result<()> {
    // Check working directory validity
    let path = if let Ok(path) = PathBuf::from(wd).canonicalize() {
        if !path.is_dir() {
            return Err(anyhow!("Invalid path"));
        }
        path
    } else {
        return Err(anyhow!("Invalid path"));
    };

    // Save old working directory
    let old_path = get_working_dir(rt).to_path_buf();
    let old_path_str = old_path.to_str().expect("failed converting to str");
    rt.env
        .set("OLDPWD", old_path_str)
        .expect("failed setting env var");

    let pwd = path.to_str().expect("failed converting to str");
    rt.env.set("PWD", pwd).expect("failed setting env var");
    rt.working_dir = path.clone();

    // Set process working directory too
    env::set_current_dir(path.clone()).expect("failed setting process current dir");

    // Run change directory hook
    if run_hook {
        let hook_ctx = ChangeDirCtx {
            old_dir: old_path,
            new_dir: path,
        };
        if let Err(e) = sh.hooks.run(sh, ctx, rt, hook_ctx) {
            error!("Error running change dir hook {e:?}");
        }
    }

    Ok(())
}

pub fn get_working_dir(rt: &Runtime) -> &Path {
    &rt.working_dir
}
