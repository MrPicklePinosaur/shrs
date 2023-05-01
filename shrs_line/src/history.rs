//! Shell history

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use thiserror::Error;

/// Trait to implement for shell history
pub trait History {
    type HistoryItem;

    /// Insert item into shell history
    fn add(&mut self, cmd: Self::HistoryItem);
    /// Remove all history entries
    fn clear(&mut self);
    // fn iter(&self) -> impl Iterator<Item = Self::HistoryItem>;
    /// Query for a history entry
    fn search(&self, query: &str) -> Option<&Self::HistoryItem>;
    /// Get number of history entries
    fn len(&self) -> usize;
    /// Get a history entry by index
    fn get(&self, i: usize) -> Option<&Self::HistoryItem>;
}

/// Default implementation of [History] that saves history in process memory
pub struct DefaultHistory {
    hist: Vec<String>,
}

impl DefaultHistory {
    pub fn new() -> Self {
        DefaultHistory { hist: vec![] }
    }
}

impl History for DefaultHistory {
    type HistoryItem = String;

    fn add(&mut self, item: Self::HistoryItem) {
        self.hist.insert(0, item);
    }

    fn clear(&mut self) {
        self.hist.clear();
    }

    // fn iter(&self) -> impl Iterator<Item = Self::HistoryItem> {
    //     todo!()
    // }

    fn search(&self, _query: &str) -> Option<&Self::HistoryItem> {
        todo!()
    }

    fn len(&self) -> usize {
        self.hist.len()
    }

    /// Get index starts at most recent (index zero is previous command)
    fn get(&self, i: usize) -> Option<&Self::HistoryItem> {
        self.hist.get(i)
    }
}

/// Store the history persistantly in a file on disk
///
/// History file is a very simple file consistaning of each history item on it's own line
// TODO potential options
// - history len
// - remove duplicates
// - only use valid commands
// - resolve alias
pub struct FileBackedHistory {
    hist: Vec<String>,
}

#[derive(Debug, Error)]
pub enum FileBackedHistoryError {
    #[error("error when opening history file {0}")]
    OpeningHistFile(std::io::Error),
}

impl FileBackedHistory {
    pub fn new(hist_file: PathBuf) -> Result<Self, FileBackedHistoryError> {
        Ok(FileBackedHistory { hist: vec![] })
    }
}

fn parse_history_file(hist_file: PathBuf) -> Result<Vec<String>, FileBackedHistoryError> {
    let handle = File::open(hist_file).map_err(|e| FileBackedHistoryError::OpeningHistFile(e))?;
    let reader = BufReader::new(handle);
    // TODO should error/terminate when a line cannot be read?
    let hist = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();
    Ok(hist)
}
