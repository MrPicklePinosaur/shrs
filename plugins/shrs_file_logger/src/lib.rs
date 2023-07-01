use std::path::PathBuf;

pub use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
};
use shrs::prelude::*;

pub struct FileLogger {
    pub path: PathBuf,
    pub level: LevelFilter,
}

impl FileLogger {
    pub fn init(&self) {
        let logfile = FileAppender::builder().build(&self.path).unwrap();

        let config = log4rs::Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(self.level))
            .unwrap();

        let _handle = log4rs::init_config(config).unwrap();
    }
}
