use std::collections::HashMap;

pub struct Program {
    pub name: String,
    pub root_cmd: Command,
    pub description: String,
}

pub struct Command {
    pub name: String,
    pub subcommands: HashMap<String, Command>,
    pub flags: Vec<Flag>,
    pub args: Vec<Arg>,
}

pub struct Flag {
    pub short: Option<char>,
    pub long: String,
    pub description: Option<String>,
}

pub struct Arg {
    // pub formatter:
}

/// Actual engine that is resposible for the completion
pub struct Engine {}
