use std::{fmt, process::ExitStatus};

use log::*;
use nix::{
    sys::{
        signal::{self, Signal},
        termios::{self, Termios},
    },
    unistd::{self, Pid},
};
use thiserror::Error;

use super::{
    process::{Process, ProcessGroup, ProcessStatus},
    util,
};
use crate::log_if_err;

#[allow(non_camel_case_types)]
pub type pid_t = i32;

#[derive(Error, Debug)]
pub enum Error {
    #[error("no such job {0}")]
    NoSuchJob(String),
}

pub trait Job {
    fn id(&self) -> JobId;
    fn input(&self) -> String;
    fn display(&self) -> String;
    fn processes(&self) -> &Vec<Box<dyn Process>>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JobId(pub u32);

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum JobStatus {
    Running,
    Stopped,
    Completed,
}

trait JobExt: Job {
    fn tmodes(&self) -> &Option<Termios>;
    fn status(&self) -> JobStatus;
}

trait AsJob {
    fn as_job(&self) -> &dyn Job;
}

impl<T: Job> AsJob for T {
    fn as_job(&self) -> &dyn Job {
        self
    }
}

#[derive(Default)]
pub struct JobManager {
    jobs: Vec<JobImpl>,
    job_count: u32,
    current_job: Option<JobId>,
}

impl JobManager {
    pub fn create_job(&mut self, input: &str, process_group: ProcessGroup) -> JobId {
        let job_id = self.get_next_job_id();
        self.jobs.push(JobImpl::new(
            job_id,
            input,
            process_group.id.map(|pgid| pgid as pid_t),
            process_group.processes,
        ));
        job_id
    }

    pub fn has_jobs(&self) -> bool {
        !self.jobs.is_empty()
    }

    pub fn get_jobs(&self) -> Vec<&dyn Job> {
        self.jobs.iter().map(|j| j.as_job()).collect()
    }

    /// Waits for job to stop or complete.
    ///
    /// This function also updates the statuses of other jobs if we receive
    /// a signal for one of their processes.
    pub fn wait_for_job(&mut self, job_id: JobId) -> anyhow::Result<Option<ExitStatus>> {
        while self.job_is_running(job_id) {
            for job in &mut self.jobs {
                job.try_wait()?;
            }
        }

        let job_index = self.find_job(job_id).expect("job not found");
        Ok(self.jobs[job_index].last_status_code())
    }

    pub fn put_job_in_foreground(
        &mut self,
        job_id: Option<JobId>,
        cont: bool,
    ) -> anyhow::Result<Option<ExitStatus>> {
        let job_id = job_id
            .or(self.current_job)
            .ok_or_else(|| Error::NoSuchJob("current".into()))?;

        dbg!("putting job [{}] in foreground", job_id);

        let _terminal_state = {
            let job_index = self
                .find_job(job_id)
                .ok_or_else(|| Error::NoSuchJob(format!("{job_id}")))?;
            self.jobs[job_index].set_last_running_in_foreground(true);
            let job_pgid = self.jobs[job_index].pgid();
            let job_tmodes = self.jobs[job_index].tmodes().clone();
            let _terminal_state = job_pgid.map(|pgid| TerminalState::new(Pid::from_raw(pgid)));

            // Send the job a continue signal if necessary
            if cont {
                if let Some(ref tmodes) = job_tmodes {
                    let temp_result = termios::tcsetattr(
                        util::get_terminal(),
                        termios::SetArg::TCSADRAIN,
                        tmodes,
                    );
                    log_if_err!(
                        temp_result,
                        "error setting terminal configuration for job ({})",
                        job_id
                    );
                }
                if let Some(ref pgid) = job_pgid {
                    signal::kill(Pid::from_raw(-pgid), Signal::SIGCONT)?;
                }
            }
            _terminal_state
        };
        self.wait_for_job(job_id)
    }

    pub fn put_job_in_background(
        &mut self,
        job_id: Option<JobId>,
        cont: bool,
    ) -> anyhow::Result<()> {
        let job_id = job_id
            .or(self.current_job)
            .ok_or_else(|| Error::NoSuchJob("current".into()))?;
        debug!("putting job [{}] in background", job_id);

        let job_pgid = {
            let job_index = self
                .find_job(job_id)
                .ok_or_else(|| Error::NoSuchJob(format!("{job_id}")))?;
            self.jobs[job_index].set_last_running_in_foreground(false);
            self.jobs[job_index].pgid()
        };

        if cont {
            if let Some(ref pgid) = job_pgid {
                signal::kill(Pid::from_raw(-pgid), Signal::SIGCONT)?;
            }
        }

        self.current_job = Some(job_id);
        Ok(())
    }

    pub fn kill_job(&mut self, job_id: JobId) -> anyhow::Result<Option<&dyn Job>> {
        if let Some(job_index) = self.find_job(job_id) {
            self.jobs[job_index].kill()?;
            Ok(Some(&self.jobs[job_index]))
        } else {
            Ok(None)
        }
    }

    /// Checks for processes that have status information available, without
    /// blocking.
    pub fn update_job_statues(&mut self) -> anyhow::Result<()> {
        for job in &mut self.jobs {
            job.try_wait()?;
        }

        Ok(())
    }

    /// Notify the user about stopped or terminated jobs and remove terminated
    /// jobs from the active job list.
    pub fn do_job_notification(&mut self) {
        let temp_result = self.update_job_statues();
        log_if_err!(temp_result, "do_job_notification");

        for job in &mut self.jobs.iter_mut() {
            if job.is_completed() && !job.last_running_in_foreground() {
                // Unnecessary to notify if the job was last running in the
                // foreground, because the user will have noticed it completed.
                println!("{}", *job);
            } else if job.is_stopped() && !job.notified_stopped_job() {
                println!("{}", *job);
                job.set_notified_stopped_job(true);
            }
        }

        // Remove completed jobs
        self.jobs.retain(|j| !j.is_completed());
    }

    fn get_next_job_id(&mut self) -> JobId {
        self.job_count += 1;
        JobId(self.job_count)
    }

    /// # Panics
    /// Panics if job is not found
    fn job_is_running(&self, job_id: JobId) -> bool {
        let job_index = self.find_job(job_id).expect("job not found");
        !self.jobs[job_index].is_stopped() && !self.jobs[job_index].is_completed()
    }

    fn find_job(&self, job_id: JobId) -> Option<usize> {
        self.jobs.iter().position(|job| job.id() == job_id)
    }
}

impl fmt::Debug for JobManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} jobs\tjob_count: {}", self.jobs.len(), self.job_count)?;
        for job in &self.jobs {
            write!(f, "{job:?}")?;
        }

        Ok(())
    }
}

impl fmt::Display for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            JobStatus::Running => write!(f, "Running"),
            JobStatus::Stopped => write!(f, "Stopped"),
            JobStatus::Completed => write!(f, "Completed"),
        }
    }
}

pub struct JobImpl {
    id: JobId,
    input: String,
    pgid: Option<pid_t>,
    processes: Vec<Box<dyn Process>>,
    last_status_code: Option<ExitStatus>,
    last_running_in_foreground: bool,
    notified_stopped_job: bool,
    tmodes: Option<Termios>,
}

impl JobImpl {
    pub fn new(
        id: JobId,
        input: &str,
        pgid: Option<pid_t>,
        processes: Vec<Box<dyn Process>>,
    ) -> Self {
        // Initialize last_status_code if possible; this prevents a completed
        // job from having a None last_status_code if all processes have
        // already completed (e.g. 'false && echo foo')
        let last_status_code = processes.iter().rev().find_map(|p| p.status_code());

        Self {
            id,
            input: input.to_string(),
            pgid,
            processes,
            last_status_code,
            last_running_in_foreground: true,
            notified_stopped_job: false,
            tmodes: termios::tcgetattr(util::get_terminal()).ok(),
        }
    }

    fn pgid(&self) -> Option<pid_t> {
        self.pgid
    }

    fn last_status_code(&self) -> Option<ExitStatus> {
        self.last_status_code
    }

    fn last_running_in_foreground(&self) -> bool {
        self.last_running_in_foreground
    }

    fn set_last_running_in_foreground(&mut self, last_running_in_foreground: bool) {
        self.last_running_in_foreground = last_running_in_foreground;
    }

    fn kill(&mut self) -> anyhow::Result<()> {
        for process in &mut self.processes {
            process.kill()?;
        }

        Ok(())
    }

    fn try_wait(&mut self) -> anyhow::Result<Option<ExitStatus>> {
        for process in &mut self.processes {
            if let Some(exit_status) = process.try_wait()? {
                // BUG: this is not actually the most recently exited process,
                // but instead the latest process in the job that has exited
                self.last_status_code = Some(exit_status);
            }
        }

        Ok(self.last_status_code)
    }

    fn notified_stopped_job(&self) -> bool {
        self.notified_stopped_job
    }

    fn set_notified_stopped_job(&mut self, notified_stopped_job: bool) {
        self.notified_stopped_job = notified_stopped_job;
    }

    fn is_stopped(&self) -> bool {
        self.processes
            .iter()
            .all(|p| p.status() == ProcessStatus::Stopped)
    }

    fn is_completed(&self) -> bool {
        self.processes
            .iter()
            .all(|p| p.status() == ProcessStatus::Completed)
    }
}

impl Job for JobImpl {
    fn id(&self) -> JobId {
        self.id
    }

    fn input(&self) -> String {
        self.input.clone()
    }

    fn display(&self) -> String {
        format!("[{}] {}\t{}", self.id, self.status(), self.input)
    }

    fn processes(&self) -> &Vec<Box<dyn Process>> {
        &self.processes
    }
}

impl JobExt for JobImpl {
    fn tmodes(&self) -> &Option<Termios> {
        &self.tmodes
    }

    fn status(&self) -> JobStatus {
        if self.is_stopped() {
            JobStatus::Stopped
        } else if self.is_completed() {
            JobStatus::Completed
        } else {
            JobStatus::Running
        }
    }
}

impl fmt::Display for JobImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}\t{}", self.id, self.status(), self.input)
    }
}

impl fmt::Debug for JobImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}\tinput: {}", self.id, self.input)
    }
}

/// RAII struct to encapsulate manipulating terminal state.
struct TerminalState {
    prev_pgid: Pid,
    prev_tmodes: Option<Termios>,
}

impl TerminalState {
    fn new(new_pgid: Pid) -> TerminalState {
        debug!("setting terminal process group to job's process group");
        let shell_terminal = util::get_terminal();
        unistd::tcsetpgrp(shell_terminal, new_pgid).unwrap();
        TerminalState {
            prev_pgid: unistd::getpgrp(),
            prev_tmodes: termios::tcgetattr(shell_terminal).ok(),
        }
    }
}

impl Drop for TerminalState {
    fn drop(&mut self) {
        debug!("putting shell back into foreground and restoring shell's terminal modes");
        let shell_terminal = util::get_terminal();
        unistd::tcsetpgrp(shell_terminal, self.prev_pgid).unwrap();
        if let Some(ref prev_tmodes) = self.prev_tmodes {
            let temp_result =
                termios::tcsetattr(shell_terminal, termios::SetArg::TCSADRAIN, prev_tmodes);
            log_if_err!(
                temp_result,
                "error restoring terminal configuration for shell"
            );
        }
    }
}
