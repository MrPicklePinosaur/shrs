use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use shrs::{
    anyhow::Result,
    plugin::Plugin,
    prelude::{History, States},
    shell::{Shell, ShellConfig},
};
use thiserror::Error;

/// Store the history persistently in a file on disk
///
/// History file is a very simple file consistaning of each history item on it's own line
// TODO potential options
// - history len
// - remove duplicates
// - only use valid commands
// - resolve alias
pub struct FileBackedHistoryState {
    hist: Vec<String>,
    hist_file: PathBuf,
    // config options
    // /// Don't keep duplicate history values
    // dedup: bool,
    // /// Max length of history to keep
    // max_length: usize,
}

#[derive(Debug, Error)]
pub enum FileBackedHistoryError {
    #[error("error when opening history file {0}")]
    OpeningHistFile(std::io::Error),
    #[error("error writing history to disk {0}")]
    Flush(std::io::Error),
}

impl FileBackedHistoryState {
    pub fn new(hist_file: PathBuf) -> Result<Self, FileBackedHistoryError> {
        let hist = parse_history_file(hist_file.clone())?;
        Ok(FileBackedHistoryState { hist, hist_file })
    }

    fn flush(&mut self) -> Result<(), FileBackedHistoryError> {
        // TODO not efficient to dedup every step
        self.dedup();
        // TODO consider keeping handle to history file open the entire time
        let handle = File::options()
            .write(true)
            .open(&self.hist_file)
            .map_err(FileBackedHistoryError::OpeningHistFile)?;
        let mut writer = BufWriter::new(handle);
        writer
            .write_all(self.hist.join("\n").as_bytes())
            .map_err(FileBackedHistoryError::Flush)?;
        writer.flush().map_err(FileBackedHistoryError::Flush)?;
        Ok(())
    }

    /// Remove duplicate entries
    fn dedup(&mut self) {
        let mut uniques = HashSet::new();
        self.hist.retain(|x| uniques.insert(x.clone()));
    }
}

pub struct FileBackedHistoryPlugin {}
impl FileBackedHistoryPlugin {
    pub fn new() -> Self {
        FileBackedHistoryPlugin {}
    }
}

impl Plugin for FileBackedHistoryPlugin {
    fn init(&self, shell: &mut ShellConfig) -> Result<()> {
        shell.states.insert(FileBackedHistoryState::new(
            shell.config_dir.join("history"),
        )?);

        shell.history = Box::new(FileBackedHistory {});
        Ok(())
    }
}

pub struct FileBackedHistory;

impl History for FileBackedHistory {
    fn add(&self, _sh: &Shell, states: &States, cmd: String) {
        let mut state = states.get_mut::<FileBackedHistoryState>();
        if !cmd.starts_with("history run") {
            state.hist.insert(0, cmd);
            // TODO consider how often we want to flush
            state.flush().unwrap();
        }
    }

    fn clear(&self, _sh: &Shell, states: &States) {
        let mut state = states.get_mut::<FileBackedHistoryState>();

        state.hist.clear();
        state.flush().unwrap();
    }

    fn len(&self, _sh: &Shell, states: &States) -> usize {
        let state = states.get_mut::<FileBackedHistoryState>();

        state.hist.len()
    }

    fn get(&self, _sh: &Shell, states: &States, i: usize) -> Option<String> {
        let state = states.get_mut::<FileBackedHistoryState>();
        state.hist.get(i).cloned()
    }
    fn items(&self, _sh: &Shell, states: &States) -> Vec<String> {
        states.get_mut::<FileBackedHistoryState>().hist.clone()
    }
}

fn parse_history_file(hist_file: PathBuf) -> Result<Vec<String>, FileBackedHistoryError> {
    let handle = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(hist_file)
        .map_err(FileBackedHistoryError::OpeningHistFile)?;
    let reader = BufReader::new(handle);
    // TODO should error/terminate when a line cannot be read?
    let hist = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    Ok(hist)
}
