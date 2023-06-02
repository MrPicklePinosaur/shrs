//! Abstraction layer for processes
use std::{
    collections::{hash_map::Iter, HashMap},
    process::Child,
};

use anyhow::anyhow;
use pino_deref::Deref;

pub type JobId = u32;

#[derive(Deref, Clone)]
pub struct ExitStatus(pub i32);

impl ExitStatus {
    pub fn success(&self) -> bool {
        self.0 == 0
    }
    pub fn code(&self) -> i32 {
        self.0
    }
}

pub struct JobInfo {
    pub child: Child,
    pub cmd: String,
}

/// Keeps track of all the current running jobs
pub struct Jobs {
    foreground: Option<Child>,
    next_id: JobId,
    jobs: HashMap<JobId, JobInfo>,
}

impl Jobs {
    pub fn new() -> Self {
        Jobs {
            next_id: 0,
            jobs: HashMap::new(),
            foreground: None,
        }
    }

    /// Add new job to be tracked
    pub fn push(&mut self, child: Child, cmd: String) {
        let next_id = self.get_next_id();
        self.jobs.insert(next_id, JobInfo { child, cmd });
    }

    pub fn iter(&self) -> Iter<'_, JobId, JobInfo> {
        self.jobs.iter()
    }

    /// Clean up finished jobs
    pub fn retain<F>(&mut self, mut exit_handler: F)
    where
        F: FnMut(ExitStatus),
    {
        self.jobs.retain(|k, v| {
            match v.child.try_wait() {
                Ok(Some(status)) => {
                    exit_handler(ExitStatus(status.code().unwrap()));
                    false
                },
                Ok(None) => true,
                Err(e) => {
                    // TODO should throw error that there was issue waiting for job to finish
                    false
                },
            }
        });
    }

    /// Increment internally used id and get the next avaliable one
    ///
    /// Careful of overflow!
    fn get_next_id(&mut self) -> JobId {
        self.next_id += 1;
        self.next_id
    }

    /// Set the current foreground process
    pub fn set_foreground(&mut self, child: Child) -> anyhow::Result<()> {
        if self.foreground.is_some() {
            // TODO move the current foreground to a background task?
            return Err(anyhow!("There is already a foreground process"));
        }
        self.foreground = Some(child);
        Ok(())
    }

    /// Wait for foreground to terminate
    pub fn wait_foreground(&mut self) -> anyhow::Result<std::process::ExitStatus> {
        match self.foreground.take() {
            Some(mut fg) => fg.wait().map_err(|e| anyhow!("{e:?}")),
            None => Err(anyhow!("No running foreground process")),
        }
    }
}
