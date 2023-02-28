// TODO could make a history trait so users can implement their own history handlers

// TODO configuration for history like max history length and if duplicates should be stored

/// Simple history that keeps the history only for as long as program is running
#[derive(Clone)]
pub struct MemHistory {
    // consider storing the parsed version of the command
    data: Vec<String>,
}

// TODO sketch up a better History library (this current one is stupid and is just a wrapper for a vec)
impl MemHistory {
    pub fn new() -> MemHistory {
        Self { data: vec![] }
    }

    /// Append command to history
    pub fn add(&mut self, cmd: String) {
        self.data.push(cmd);
    }

    /// Wipe history
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the last command that was executed
    pub fn latest(&self) -> Option<&String> {
        if self.data.is_empty() {
            return None;
        }
        self.data.get(self.data.len() - 1)
    }

    /// Get entire history
    pub fn all(&self) -> &Vec<String> {
        &self.data
    }

    /// Query history with filters and tags
    pub fn search(&self, query: &str) {
        todo!()
    }
}

/*
impl reedline::History for MemHistory {
    fn save(&mut self, h: reedline::HistoryItem) -> anyhow::Result<reedline::HistoryItem> {
    Ok(self.add(h.command_line))
    }

    fn load(&self, id: reedline::HistoryItemId) -> anyhow::Result<reedline::HistoryItem> {
        todo!()
    }

    fn count(&self, query: reedline::SearchQuery) -> anyhow::Result<i64> {
        todo!()
    }

    fn search(&self, query: reedline::SearchQuery) -> anyhow::Result<Vec<reedline::HistoryItem>> {
        todo!()
    }

    fn update(
        &mut self,
        id: reedline::HistoryItemId,
        updater: &dyn Fn(reedline::HistoryItem) -> reedline::HistoryItem,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn clear(&mut self) -> anyhow::Result<()> {
        todo!()
    }

    fn delete(&mut self, h: reedline::HistoryItemId) -> anyhow::Result<()> {
        todo!()
    }

    fn sync(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

*/
