use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

use super::parser;

/// User defined command that gets ran when we wish to print the prompt
fn prompt_command() {
    print!("> ");
    stdout().flush();
}

/// User defined command for formatting shell error messages
fn error_command() {}

pub struct Shell {}

impl Shell {
    pub fn new() -> Self {
        Shell {}
    }

    pub fn run(&self) {
        loop {
            prompt_command();

            let mut line = String::new();
            if let Err(e) = stdin().read_line(&mut line) {
                continue;
            }

            // let mut parser = parser::ParserContext::new();
            // if let Err(e) = parser.parse(&line) {
            //     eprintln!("{}", e);
            // }

            // let mut parts = line.trim().split_whitespace();
            // let cmd = parts.next().unwrap();
            // let args = parts.collect();

            // self.run_command(cmd, &args);
        }
    }

    fn run_command(&self, cmd: &str, args: &Vec<&str>) {
        match cmd {
            "cd" => self.run_cd_command(args),
            "exit" => self.run_exit_command(args),
            _ => self.run_external_command(cmd, args),
        };
    }

    fn run_cd_command(&self, args: &Vec<&str>) {
        // if empty default to root (for now)
        let raw_path = args.get(0).unwrap_or(&"/");
        let path = Path::new(raw_path);
        if let Err(e) = env::set_current_dir(path) {
            eprintln!("{}", e);
        }
    }

    fn run_exit_command(&self, args: &Vec<&str>) {
        std::process::exit(0);
    }

    fn run_external_command(&self, cmd: &str, args: &Vec<&str>) {
        let child = Command::new(cmd).args(args).spawn();

        match child {
            Ok(mut child) => {
                child.wait();
            },
            Err(e) => {
                eprintln!("{}", e);
            },
        };
    }
}
