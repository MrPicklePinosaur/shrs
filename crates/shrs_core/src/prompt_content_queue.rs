//! Queue for automatic insertion into prompt
use std::collections::VecDeque;
/// Holds the content to be inserted
pub struct PromptContent {
    pub content: String,
    ///The command will automatically run if true
    pub auto_run: bool,
}
impl PromptContent {
    pub fn new(content: String, auto_run: bool) -> Self {
        Self { content, auto_run }
    }
}
pub struct PromptContentQueue {
    queue: VecDeque<PromptContent>,
}
impl PromptContentQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    ///Push content onto the back of queue
    pub fn push(&mut self, value: PromptContent) {
        self.queue.push_back(value)
    }
    ///Pop from front of queue
    pub fn pop(&mut self) -> Option<PromptContent> {
        self.queue.pop_front()
    }
}
