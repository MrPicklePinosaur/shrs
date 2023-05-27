//!
//!
//! A lot of this implementation is based off of https://www.gnu.org/software/libc/manual/html_node/Initializing-the-Shell.html
use std::{
    io::{stdin, Stdin},
    os::fd::{AsRawFd, RawFd},
};

use nix::{
    libc::STDIN_FILENO,
    sys::{
        signal::{kill, signal, sigprocmask, SigHandler, SigmaskHow, Signal, Signal::SIGTTIN},
        signalfd::SigSet,
    },
    unistd::{getpgrp, getpid, isatty, setpgid, tcgetpgrp, tcsetpgrp},
};

// pub struct Shell {

// }

/// Initialize job control for the shell
pub fn init_shell() -> Result<(), std::io::Error> {
    // Check if the current shell is allowed to run it's own job control
    let shell_term = STDIN_FILENO;

    if !isatty(shell_term)? {
        return Ok(());
    }

    // Wait until parent puts us into foreground
    while tcgetpgrp(shell_term)? != getpgrp() {
        // SIGTTIN tells process to suspend since it's not in foreground
        kill(getpgrp(), SIGTTIN)?;
    }

    // Ignore interactive and job control signals
    // TODO double check correctness of unsafe code
    unsafe {
        signal(Signal::SIGINT, SigHandler::SigIgn);
        signal(Signal::SIGQUIT, SigHandler::SigIgn);
        signal(Signal::SIGTSTP, SigHandler::SigIgn);
        signal(Signal::SIGTTIN, SigHandler::SigIgn);
        signal(Signal::SIGTTOU, SigHandler::SigIgn);
        signal(Signal::SIGCHLD, SigHandler::SigIgn);
    };

    // Put self in own process group
    let shell_pid = getpid();
    setpgid(shell_pid, shell_pid)?;
    tcsetpgrp(shell_term, shell_pid)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::init_shell;

    #[test]
    fn init() {
        init_shell().unwrap();
    }
}
