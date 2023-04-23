//! Custom implementation of log facade
//!
//!

use log::{Level, Log, Metadata, Record, SetLoggerError};

struct Logger {}

impl Logger {
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(self))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        match record.level() {
            Level::Error => todo!(),
            Level::Warn => todo!(),
            Level::Info => todo!(),
            Level::Debug => todo!(),
            Level::Trace => todo!(),
        }
    }

    fn flush(&self) {}
}
