//! Abstraction layer for processes
use std::{
    collections::{hash_map::Iter, HashMap},
    process::Child,
    time::{Duration, Instant},
};

pub type JobId = u32;

#[derive(Clone)]
pub struct ExitInfo {
    pub status: i32,
    pub job_duration: Duration,
}

impl ExitInfo {
    pub fn success(&self) -> bool {
        self.status == 0
    }
    pub fn code(&self) -> i32 {
        self.status
    }
}
pub struct JobTimer {
    pub start_time: Option<Instant>,
    pub job_duration: Option<Duration>,
}
impl JobTimer {
    pub fn end_job_timer(&mut self) {
        self.job_duration = Some(self.start_time.unwrap().elapsed());
    }
    pub fn init(&mut self) {
        self.start_time = Some(Instant::now());
    }
    pub fn new() -> Self {
        JobTimer {
            start_time: None,
            job_duration: None,
        }
    }
}

pub struct JobInfo {
    pub child: Child,
    pub cmd: String,
    pub timer: JobTimer,
    pub exit_status: Option<ExitInfo>,
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
        let mut timer = JobTimer::new();
        timer.init();
        self.jobs.insert(
            next_id,
            JobInfo {
                child,
                cmd,
                timer,
                exit_status: None,
            },
        );
    }

    pub fn iter(&self) -> Iter<'_, JobId, JobInfo> {
        self.jobs.iter()
    }

    /// Clean up finished jobs
    pub fn retain<F>(&mut self, mut exit_handler: F)
    where
        F: FnMut(ExitInfo),
    {
        self.jobs.retain(|k, v| {
            match v.child.try_wait() {
                Ok(Some(status)) => {
                    v.timer.end_job_timer();
                    exit_handler(ExitInfo {
                        job_duration: v.timer.job_duration.unwrap(),
                        status: status.code().unwrap(),
                    });
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
