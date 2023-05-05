use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct ProcessTimer {
    pub init_time: Instant,
    cmd_start_time: Option<Instant>,

    //should it be 0 or optional
    pub prev_cmd_time: Option<Duration>,
}
impl ProcessTimer {
    pub fn new() -> Self {
        ProcessTimer {
            init_time: Instant::now(),
            prev_cmd_time: None,
            cmd_start_time: None,
        }
    }
    pub fn start_cmd_timer(&mut self) {
        self.cmd_start_time = Some(Instant::now());
    }
    pub fn end_cmd_timer(&mut self) {
        //fix force unwrap
        match self.cmd_start_time {
            Some(start_time) => self.prev_cmd_time = Some(start_time.elapsed()),
            None => eprintln!("No command has been started yet"),
        }
        self.cmd_start_time = None
    }
}
