pub struct Runtime {}

pub struct PipeNode {}

pub struct CommandNode {
    pub cmd_name: String,
    pub args: Vec<String>,
}

impl CommandNode {
    pub fn eval(&self) {}
}
