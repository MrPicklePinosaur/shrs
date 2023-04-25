pub struct CompletionCtx {
    /// The currently entered line split by arguments
    ///
    /// The cursor position is after the very last argument
    line: Vec<String>,
}

impl CompletionCtx {
    pub fn new(line: Vec<String>) -> Self {
        Self { line }
    }

    /// Get the name of the command
    pub fn cmd_name(&self) -> Option<&String> {
        self.line.get(0)
    }

    /// Get the word that the user is currently typing
    pub fn cur_word(&self) -> Option<&String> {
        self.line.last()
    }
}
