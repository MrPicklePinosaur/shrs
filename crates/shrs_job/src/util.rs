use std::os::fd::{AsRawFd, RawFd};

use nix::{
    sys::signal::{self, SigHandler, Signal},
    unistd::{self, Pid},
};

use super::job::pid_t;

#[macro_export]
macro_rules! log_if_err {
    ($result:expr) => {{
        if let Err(e) = $result {
            log::error!("{}", e);
        }
    }};
    ($result:expr, $fmt:expr) => {{
        if let Err(e) = $result {
            log::error!(concat!($fmt, ": {}"), e);
        }
    }};
    ($result:expr, $fmt:expr, $($arg:tt)*) => {{
        if let Err(e) = $result {
            log::error!(concat!($fmt, ": {}"), $($arg)*, e);
        }
    }};
}

#[macro_export]
macro_rules! warn_if_err {
    ($result:expr) => {{
        if let Err(e) = $result {
            log::warn!("{}", e);
        }
    }};
    ($result:expr, $fmt:expr) => {{
        if let Err(e) = $result {
            log::warn!(concat!($fmt, ": {}"), e);
        }
    }};
    ($result:expr, $fmt:expr, $($arg:tt)*) => {{
        if let Err(e) = $result {
            log::warn!(concat!($fmt, ": {}"), $($arg)*, e);
        }
    }};
}

pub fn get_terminal() -> RawFd {
    std::io::stdin().as_raw_fd()
}

pub fn initialize_job_control() -> anyhow::Result<()> {
    let shell_terminal = get_terminal();

    // Loop until the shell is in the foreground
    loop {
        let shell_pgid = unistd::getpgrp();
        if unistd::tcgetpgrp(shell_terminal)? == shell_pgid {
            break;
        } else {
            signal::kill(Pid::from_raw(-pid_t::from(shell_pgid)), Signal::SIGTTIN).unwrap();
        }
    }

    // Ignore interactive and job-control signals
    unsafe {
        signal::signal(Signal::SIGINT, SigHandler::SigIgn).unwrap();
        signal::signal(Signal::SIGQUIT, SigHandler::SigIgn).unwrap();
        signal::signal(Signal::SIGTSTP, SigHandler::SigIgn).unwrap();
        signal::signal(Signal::SIGTTIN, SigHandler::SigIgn).unwrap();
        signal::signal(Signal::SIGTTOU, SigHandler::SigIgn).unwrap();
    }

    // Put ourselves in our own process group
    let shell_pgid = Pid::this();
    unistd::setpgid(shell_pgid, shell_pgid)?;

    // Grab control of the terminal and save default terminal attributes
    let shell_terminal = get_terminal();
    let temp_result = unistd::tcsetpgrp(shell_terminal, shell_pgid);
    log_if_err!(temp_result, "failed to grab control of terminal");

    Ok(())
}
