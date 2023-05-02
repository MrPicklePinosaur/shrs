//! Shell history

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use crossterm::QueueableCommand;
use thiserror::Error;

/// Trait to implement for shell history
pub trait History {
    type HistoryItem;

    /// Insert item into shell history
    fn add(&mut self, cmd: Self::HistoryItem);
    /// Remove all history entries
    fn clear(&mut self);
    /// Query for a history entry
    fn search(&self, query: &str) -> Option<&Self::HistoryItem>;
    /// Get number of history entries
    fn len(&self) -> usize;
    /// Get a history entry by index
    fn get(&self, i: usize) -> Option<&Self::HistoryItem>;
    fn iter(&self) -> HistoryIter<Self::HistoryItem>;
}

pub struct HistoryIter<'a, It>(Box<dyn Iterator<Item = &'a It>>);

impl<'a, It> Iterator for HistoryIter<'a, It> {
    type Item = &'a It;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
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

    fn iter(&self) -> HistoryIter<Self::HistoryItem> {
        HistoryIter(Box::new(self.hist.iter()))
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
    hist_file: PathBuf,
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

    fn flush(&self) -> Result<(), FileBackedHistoryError> {
        // TODO consider keeping handle to history file open the entire time
        let handle = File::options()
            .write(true)
            .open(&self.hist_file)
            .map_err(|e| FileBackedHistoryError::OpeningHistFile(e))?;
        let mut writer = BufWriter::new(handle);
        writer
            .write_all(self.hist.join("\n").as_bytes())
            .map_err(|e| FileBackedHistoryError::Flush(e))?;
        writer
            .flush()
            .map_err(|e| FileBackedHistoryError::Flush(e))?;
        Ok(())
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

    fn search(&self, _query: &str) -> Option<&Self::HistoryItem> {
        todo!()
    }

    fn len(&self) -> usize {
        self.hist.len()
    }

    fn get(&self, i: usize) -> Option<&Self::HistoryItem> {
        self.hist.get(i)
    }

    fn iter(&self) -> HistoryIter<Self::HistoryItem> {
        HistoryIter(Box::new(self.hist.iter()))
    }
}

fn parse_history_file(hist_file: PathBuf) -> Result<Vec<String>, FileBackedHistoryError> {
    let handle = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(hist_file)
        .map_err(|e| FileBackedHistoryError::OpeningHistFile(e))?;
    let reader = BufReader::new(handle);
    // TODO should error/terminate when a line cannot be read?
    let hist = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();
    Ok(hist)
}
