//! Process management

use std::{
    ffi::{CStr, CString},
    io::{stdin, Stdin},
    os::fd::{AsRawFd, RawFd},
    process::exit,
};

use nix::{
    libc::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO},
    sys::{
        signal::{signal, sigprocmask, SigHandler, SigmaskHow, Signal},
        signalfd::SigSet,
    },
    unistd::{close, dup2, execvp, fork, getpid, isatty, setpgid, tcsetpgrp, ForkResult, Pid},
};

pub enum Pgid {
    /// Pgid of current corresponds to using the same Pgid as the current group is using
    Current,
    /// A specific Pgid
    Pgid(Pid),
}

/// A single OS process
pub struct Process {
    /// Process id
    pub pid: Pid,
    /// List of args to be passed to process
    pub argv: Vec<String>,
}

/// A job corresponds to a pipeline of processes
pub struct Job {
    /// Process group id
    pub pgid: Pid,
    /// Pipeline of processes
    pipeline: Vec<Process>,
    pub stdin: RawFd,
    pub stdout: RawFd,
    pub stderr: RawFd,
    pub is_foreground: bool,
}

impl Process {
    pub fn run(
        &self,
        pgid: Pgid,
        is_foreground: bool,
        stdin: RawFd,
        stdout: RawFd,
        stderr: RawFd,
    ) -> Result<(), std::io::Error> {
        // If interactive need to give the current process control of the tty
        let shell_term = STDIN_FILENO;
        if isatty(shell_term)? {
            let pid = getpid();
            let new_pgid = match pgid {
                Pgid::Current => pid,
                Pgid::Pgid(pgid) => pgid,
            };
            setpgid(pid, new_pgid)?;

            // If process is being launched by foreground job, we also need the process to be in
            // the foreground
            if is_foreground {
                tcsetpgrp(shell_term, new_pgid)?;
            }

            // Reset signals
            unsafe {
                signal(Signal::SIGINT, SigHandler::SigIgn);
                signal(Signal::SIGQUIT, SigHandler::SigIgn);
                signal(Signal::SIGTSTP, SigHandler::SigIgn);
                signal(Signal::SIGTTIN, SigHandler::SigIgn);
                signal(Signal::SIGTTOU, SigHandler::SigIgn);
                signal(Signal::SIGCHLD, SigHandler::SigIgn);
            };
        }

        // Set stdio of new process
        if stdin != STDIN_FILENO {
            dup2(stdin, STDIN_FILENO)?;
            close(stdin)?;
        }
        if stdout != STDOUT_FILENO {
            dup2(stdout, STDOUT_FILENO)?;
            close(stdout)?;
        }
        if stderr != STDERR_FILENO {
            dup2(stderr, STDERR_FILENO)?;
            close(stderr)?;
        }

        // We can fork now
        let filename = self.argv.get(0).unwrap();
        let args = self
            .argv
            .iter()
            .map(|s| CString::new(s.clone()).unwrap())
            .collect::<Vec<_>>();
        execvp(&CString::new(filename.clone()).unwrap(), &args)?;
        exit(1);
    }
}

impl Job {
    pub fn run(&self) -> Result<(), std::io::Error> {
        for process in self.pipeline.iter() {
            // set up pipes

            // fork the child
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child }) => {},
                Ok(ForkResult::Child) => {
                    process.run(
                        Pgid::Pgid(self.pgid),
                        self.is_foreground,
                        self.stdin,
                        self.stdout,
                        self.stderr,
                    )?;
                },
                Err(_) => todo!(),
            }

            // clean up each pipe
        }
        Ok(())
    }

    pub fn leader(&self) -> &Process {
        // Job should always have at least one process in the pipeline
        &self.pipeline.get(0).unwrap()
    }
}
