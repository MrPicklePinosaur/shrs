//! Abstraction layer for processes
use std::{
    collections::{hash_map::Iter, HashMap},
    process::{Child, ExitStatus},
};

use anyhow::anyhow;

pub type JobId = u32;

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

impl Default for Jobs {
    fn default() -> Self {
        Jobs {
            next_id: 0,
            jobs: HashMap::new(),
            foreground: None,
        }
    }
}

impl Jobs {
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
        self.jobs.retain(|_, v| {
            match v.child.try_wait() {
                Ok(Some(status)) => {
                    exit_handler(status);
                    false
                },
                Ok(None) => true,
                Err(_) => {
                    // TODO should throw error that there was issue waiting for job to finish
                    false
                },
            }
        });
    }

    /// Increment internally used id and get the next available one
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
