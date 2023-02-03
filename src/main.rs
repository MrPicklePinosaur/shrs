use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn main() {
    loop {
        print!("> ");
        stdout().flush();

        let mut line = String::new();
        if let Err(e) = stdin().read_line(&mut line) {
            continue;
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

fn run_cd_command(args: &Vec<&str>) {}

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
