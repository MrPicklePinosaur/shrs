use std::collections::VecDeque;

pub struct PromptContent {
    pub content: String,
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
    pub fn push(&mut self, value: PromptContent) {
        self.queue.push_back(value)
    }
    pub fn pop(&mut self) -> Option<PromptContent> {
        self.queue.pop_front()
    }
}
