//! Abstraction layer for processes

use std::{
    collections::{hash_map::Iter, HashMap},
    process::Child,
};

pub type JobId = u32;

pub struct ExitStatus(pub i32);

impl ExitStatus {
    pub fn success(&self) -> bool {
        self.0 == 0
    }
}

/// Keeps track of all the current running jobs
pub struct Jobs {
    next_id: JobId,
    jobs: HashMap<JobId, Child>,
}

impl Jobs {
    pub fn new() -> Self {
        Jobs {
            next_id: 0,
            jobs: HashMap::new(),
        }
    }

    /// Add new job to be tracked
    pub fn push(&mut self, child: Child) {
        let next_id = self.get_next_id();
        self.jobs.insert(next_id, child);
    }

    pub fn iter(&self) -> Iter<'_, JobId, Child> {
        self.jobs.iter()
    }

    /// Clean up finished jobs
    pub fn retain(&mut self) {
        // self.jobs.retain(|job| {
        // });
        todo!()
    }

    fn get_next_id(&mut self) -> JobId {
        self.next_id += 1;
        self.next_id
    }
}
