//! Shell history

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
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

    /// Check if the history is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Default implementation of [History] that saves history in process memory
#[derive(Default)]
pub struct DefaultHistory {
    hist: Vec<String>,
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

/// Store the history persistently in a file on disk
///
/// History file is a very simple file consistaning of each history item on it's own line
// TODO potential options
// - history len
// - remove duplicates
// - only use valid commands
// - resolve alias
pub struct FileBackedHistory {
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

impl FileBackedHistory {
    pub fn new(hist_file: PathBuf) -> Result<Self, FileBackedHistoryError> {
        let hist = parse_history_file(hist_file.clone())?;
        Ok(FileBackedHistory { hist, hist_file })
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

impl History for FileBackedHistory {
    type HistoryItem = String;

    fn add(&mut self, item: Self::HistoryItem) {
        self.hist.insert(0, item);
        // TODO consider how often we want to flush
        self.flush().unwrap();
    }

    fn clear(&mut self) {
        self.hist.clear();
        self.flush().unwrap();
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

    fn get(&self, i: usize) -> Option<&Self::HistoryItem> {
        self.hist.get(i)
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
