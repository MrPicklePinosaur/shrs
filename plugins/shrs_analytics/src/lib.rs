use std::{collections::HashMap, path::PathBuf};

use builtin::AnalyticsBuiltin;
use shrs::{anyhow::Result, prelude::*};

mod builtin;

// TODO
// Builtin to retrieve analytics

// Metrics:
// command execute count
// most common directory
// Shell usage time
// Suggested aliases based off common commands
// Make stored data easily accessible to other plugins so that they can do smart things
// Maybe predict what cd is going to happen based on how often user cds from one dir to the other

// Hooks to collect analytics

struct AnalyticsState {
    commands: HashMap<String, u32>,
    dirs: HashMap<PathBuf, u32>,
}

impl AnalyticsState {
    pub fn new() -> Self {
        AnalyticsState {
            commands: HashMap::new(),
            dirs: HashMap::new(),
        }
    }
}

pub struct AnalyticsPlugin {}

impl AnalyticsPlugin {
    pub fn new() -> Self {
        AnalyticsPlugin {}
    }
}

impl Plugin for AnalyticsPlugin {
    fn init(&self, shell: &mut ShellConfig) -> Result<()> {
        shell.builtins.insert("analytics", AnalyticsBuiltin);
        shell.hooks.register(record_dir_change);
        shell.hooks.register(most_common_commands);
        shell.state.insert(AnalyticsState::new());

        Ok(())
    }
}

fn record_dir_change(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    cd_ctx: &ChangeDirCtx,
) -> Result<()> {
    Ok(())
}

fn most_common_commands(
    sh: &Shell,
    ctx: &mut Context,
    rt: &mut Runtime,
    cmd_ctx: &BeforeCommandCtx,
) -> anyhow::Result<()> {
    // TODO maybe read commands from history too?
    ctx.state.get_mut::<AnalyticsState>().map(|state| {
        // add to most used commands

        // TODO IFS
        let cmd_name = cmd_ctx.command.split(' ').next().unwrap().to_string();
        if let Some(count) = state.commands.get(&cmd_name) {
            state.commands.insert(cmd_name, count + 1);
        } else {
            state.commands.insert(cmd_name, 1);
        }

        // add to most used dirs
        if let Some(count) = state.dirs.get(&rt.working_dir) {
            state.dirs.insert(rt.working_dir.clone(), count + 1);
        } else {
            state.dirs.insert(rt.working_dir.clone(), 1);
        }
    });
    Ok(())
}
