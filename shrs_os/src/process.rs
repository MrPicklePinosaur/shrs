//! Process management

use std::os::fd::RawFd;

use nix::unistd::Pid;

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
    pub pipeline: Vec<Process>,
    pub stdin: RawFd,
    pub stdout: RawFd,
    pub stderr: RawFd,
}
