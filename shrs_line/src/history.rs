//! Shell history

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
        DefaultHistory {
            hist: vec!["aaa".into(), "bbb".into(), "ccc".into()],
        }
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
