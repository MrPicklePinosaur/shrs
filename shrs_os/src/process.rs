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
    /// All of the processes in this job
    pub proceses: Vec<Process>,
}

/// Execution context for a process
pub struct Context {
    pub stdin: RawFd,
    pub stdout: RawFd,
    pub stderr: RawFd,
    /// Is the current job running in the foreground
    pub is_foreground: bool,
    /// Is the shell in interactive mode
    pub is_interactive: bool,
}

pub fn run_job(argv: Vec<String>, pgid: Pgid, ctx: &Context) -> Result<(), std::io::Error> {
    // If interactive need to give the current process control of the tty
    let shell_term = STDIN_FILENO;
    if ctx.is_interactive {
        let pid = getpid();
        let new_pgid = match pgid {
            Pgid::Current => pid,
            Pgid::Pgid(pgid) => pgid,
        };
        setpgid(pid, new_pgid)?;

        // If process is being launched by foreground job, we also need the process to be in
        // the foreground
        if ctx.is_foreground {
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
    if ctx.stdin != STDIN_FILENO {
        dup2(ctx.stdin, STDIN_FILENO)?;
        close(ctx.stdin)?;
    }
    if ctx.stdout != STDOUT_FILENO {
        dup2(ctx.stdout, STDOUT_FILENO)?;
        close(ctx.stdout)?;
    }
    if ctx.stderr != STDERR_FILENO {
        dup2(ctx.stderr, STDERR_FILENO)?;
        close(ctx.stderr)?;
    }

    // We can fork now
    let filename = argv.get(0).unwrap();
    let args = argv
        .iter()
        .map(|s| CString::new(s.clone()).unwrap())
        .collect::<Vec<_>>();
    execvp(&CString::new(filename.clone()).unwrap(), &args)?;
    exit(1);
}

impl Job {
    /*
    pub fn run(&self, ctx: &Context) -> Result<(), std::io::Error> {
        for process in self.pipeline.iter() {
            // set up pipes

            // fork the child
            match unsafe { fork() } {
                Ok(ForkResult::Parent { child }) => {},
                Ok(ForkResult::Child) => {
                    process.run(
                        Pgid::Pgid(self.pgid),
                        ctx
                    )?;
                },
                Err(_) => todo!(),
            }

            // clean up each pipe
        }
        Ok(())
    }
    */

    pub fn leader(&self) -> &Process {
        // Job should always have at least one process in the pipeline
        // &self.pipeline.get(0).unwrap()
        todo!()
    }
}
