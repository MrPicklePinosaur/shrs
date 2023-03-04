pub trait Prompt {
    fn prompt_left(&self) -> String;
}

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
