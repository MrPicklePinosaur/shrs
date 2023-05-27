use std::sync::{atomic::AtomicBool, Arc};

use signal_hook::{consts::*, flag};

pub struct Signals {
    pub int: Arc<AtomicBool>,
}

impl Signals {
    pub fn new() -> Result<Self, std::io::Error> {
        let int = Arc::new(AtomicBool::new(false));

        flag::register(SIGINT, Arc::clone(&int))?;

        Ok(Self { int })
    }
}
