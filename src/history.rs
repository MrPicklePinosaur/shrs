// TODO could make a history trait so users can implement their own history handlers

// TODO configuration for history like max history length and if duplicates should be stored

pub struct History {
    // consider storing the parsed version of the command
    data: Vec<String>,
}

// TODO sketch up a better History library (this current one is stupid and is just a wrapper for a vec)
impl History {
    pub fn new() -> History {
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
