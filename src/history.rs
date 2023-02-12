pub struct History {
    // consider storing the parsed version of the command
    data: Vec<String>,
}

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
}
