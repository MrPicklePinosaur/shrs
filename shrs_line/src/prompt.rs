//! Shell prompt

pub trait Prompt {
    fn prompt_left(&self) -> String;
}

/// Default implementation for [Prompt]
pub struct DefaultPrompt {}

impl DefaultPrompt {
    pub fn new() -> Self {
        DefaultPrompt {}
    }
}

impl Prompt for DefaultPrompt {
    fn prompt_left(&self) -> String {
        String::from("> ")
    }
}
