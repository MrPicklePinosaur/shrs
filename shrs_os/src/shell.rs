//!
//!
//! A lot of this implementation is based off of https://www.gnu.org/software/libc/manual/html_node/Initializing-the-Shell.html
use std::{
    io::{stdin, Stdin},
    os::fd::{AsRawFd, RawFd},
};

use nix::{
    sys::{
        signal::{kill, sigprocmask, SigmaskHow, Signal, Signal::SIGTTIN},
        signalfd::SigSet,
    },
    unistd::{getpgrp, getpid, isatty, setpgid, tcgetpgrp, tcsetpgrp},
};

/// Initialize job control for the shell
pub fn init_shell() -> Result<(), std::io::Error> {
    // Check if the current shell is allowed to run it's own job control
    let shell_term = stdin().as_raw_fd();

    if !isatty(shell_term)? {
        return Ok(());
    }

    // Wait until parent puts us into foreground
    while tcgetpgrp(shell_term)? != getpgrp() {
        // SIGTTIN tells process to suspend since it's not in foreground
        kill(getpgrp(), SIGTTIN)?;
    }

    // Ignore interactive and job control signals
    let mut sigset = SigSet::empty();
    sigset.add(Signal::SIGINT);
    sigset.add(Signal::SIGQUIT);
    sigset.add(Signal::SIGTSTP);
    sigset.add(Signal::SIGTTIN);
    sigset.add(Signal::SIGTTOU);
    sigset.add(Signal::SIGCHLD);
    sigprocmask(SigmaskHow::SIG_BLOCK, Some(&sigset), None)?;

    // Put self in own process group
    let shell_pid = getpid();
    setpgid(shell_pid, shell_pid)?;
    tcsetpgrp(shell_term, shell_pid)?;

    Ok(())
}
