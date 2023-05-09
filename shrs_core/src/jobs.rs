//! Abstraction layer for processes
use std::{
    collections::{hash_map::Iter, HashMap},
    process::Child,
};

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
    next_id: JobId,
    jobs: HashMap<JobId, JobInfo>,
}

impl Jobs {
    pub fn new() -> Self {
        Jobs {
            next_id: 0,
            jobs: HashMap::new(),
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
}
