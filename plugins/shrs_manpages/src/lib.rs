use std::process::{Command, Stdio};

use shrs::prelude::*;

pub struct ManPagesPlugin {
    /// The command to use to open the man page
    man_command: Option<String>,
}

/// Grab the current line and attempt to open man page of command
pub fn open_manpage(state: &mut LineStateBundle) {
    // TODO IFS
    let full_command = state.line.get_full_command();
    let Some(command) = full_command.split(' ').next() else { return; };
    println!("full  command '{command}'");

    // TODO Spawn man command and pass `command` to it as the man page to open 
    Command::new("man").arg(command).spawn().unwrap();
}

// TODO i don't think we actually need an entire plugin to do this
impl ManPagesPlugin {

    /// Initialize the man pages plugin to use the `man` command by default.
    /// If you wish to specify a different man command, use [from_man_command].
    pub fn new() -> Self {
        ManPagesPlugin {
            man_command: None
        }
    }

    /// Initialize the man pages plugin using a specific command to use to open man pages
    pub fn from_man_command<S: ToString>(man_command: S) -> Self {
        ManPagesPlugin {
            man_command: Some(man_command.to_string())
        }
    }
}

impl Plugin for ManPagesPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        // TODO insert keybinding in plugin init or let user do it themselves?
        // TODO should include config for what to bind keybinding function to?
        Ok(())
    }


    fn meta(&self) -> PluginMeta {
        PluginMeta::new("manpages", "keybinding to open manpage for currently typed command", None)
    }
}
