use std::path::PathBuf;

use shrs::prelude::*;
use shrs_file_logger::{FileLogger, LevelFilter};

fn main() {
    let logger = FileLogger {
        path: PathBuf::from("/tmp/shrs_log"),
        level: LevelFilter::Debug,
    };

    logger.init().unwrap();

    let _readline = LineBuilder::default().build().unwrap();

    let myshell = ShellBuilder::default().build().unwrap();

    myshell.run().expect("Error when running shell");
}
