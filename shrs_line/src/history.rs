pub trait History {
    type HistoryItem;

    fn add(&mut self, cmd: Self::HistoryItem);
    fn clear(&mut self);
    // fn iter(&self) -> impl Iterator<Item = Self::HistoryItem>;
    fn search(&self, query: &str) -> Option<&Self::HistoryItem>;
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> Option<&Self::HistoryItem>;
}

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

    fn search(&self, query: &str) -> Option<&Self::HistoryItem> {
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
