use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;

fn main() {
    run();
}

fn prompt_command() {
    print!("> ");
    stdout().flush();
}

fn run() {
    loop {
        prompt_command();

        let mut line = String::new();
        if let Err(e) = stdin().read_line(&mut line) {
            continue;
        }

        let mut parser = parser::ParserContext::new();
        if let Err(e) = parser.parse(&line) {
            eprintln!("{}", e);
        }

        let mut parts = line.trim().split_whitespace();
        let cmd = parts.next().unwrap();
        let args = parts.collect();

        run_command(cmd, &args);
    }
}

fn run_command(cmd: &str, args: &Vec<&str>) {
    match cmd {
        "cd" => run_cd_command(args),
        "exit" => run_exit_command(args),
        _ => run_external_command(cmd, args),
    };
}

fn run_cd_command(args: &Vec<&str>) {
    // if empty default to root (for now)
    let raw_path = args.get(0).unwrap_or(&"/");
    let path = Path::new(raw_path);
    if let Err(e) = env::set_current_dir(path) {
        eprintln!("{}", e);
    }
}

fn run_exit_command(args: &Vec<&str>) {
    std::process::exit(0);
}

fn run_external_command(cmd: &str, args: &Vec<&str>) {
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
