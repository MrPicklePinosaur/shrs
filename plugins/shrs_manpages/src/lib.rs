use std::{process::{Command, Stdio}, ffi::OsStr};

use shrs::prelude::*;

/// Open a man page for the currently typed command using the `man` command by default.
/// If you wish to specify a different man command, use [open_manpage_with].
pub fn open_manpage(state: &mut LineStateBundle) {
    _open_manpage(state, "man")
}

pub fn open_manpage_with<S: AsRef<OsStr>>(state: &mut LineStateBundle,man_command: S) {
    _open_manpage(state, man_command)
}

/// Grab the current line and attempt to open man page of command
fn _open_manpage<S: AsRef<OsStr>>(state: &mut LineStateBundle, man_command: S){
    // TODO IFS
    let full_command = state.line.get_full_command();
    let Some(command) = full_command.split(' ').next() else { return; };

    // Spawn man command and pass `command` to it as the man page to open 
    Command::new(man_command).arg(command).spawn().unwrap();

    // TODO: the old cursor buffer isn't actually preserved after executing the command.
    // so we need to save the old line and restore it after
    state.line.cb.clear();
    // state.line.cb.insert(cursor_buffer::Location::Front(), &full_command);

    // TODO after handling keybinding it seems that the line accepts the contents, so we
    // automatically run the command that was present before - open issue to allow keybindings to
    // decide if the line should accept after the keybinding or not
}
