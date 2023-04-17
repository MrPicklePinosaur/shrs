//! Shell prompt

pub trait Prompt {
    fn prompt_left(&self) -> String;
    fn prompt_right(&self) -> String;
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

    fn prompt_right(&self) -> String {
        String::new()
    }
}
