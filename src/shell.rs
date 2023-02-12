use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::{Child, Output, Stdio},
};

use anyhow::anyhow;

use crate::{ast, parser};

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

    pub fn run(&self) -> anyhow::Result<()> {
        loop {
            prompt_command();

            let mut line = String::new();
            if let Err(e) = stdin().read_line(&mut line) {
                continue;
            }

            let mut parser = parser::ParserContext::new();
            match parser.parse(&line) {
                Ok(cmd) => {
                    let cmd_handle = self.eval_command(cmd, Stdio::inherit())?;
                    let cmd_output = cmd_handle.wait_with_output()?;
                    println!("{:?}", std::str::from_utf8(&cmd_output.stdout)?);
                },
                Err(e) => {
                    eprintln!("{}", e);
                },
            }
        }
    }

    fn eval_command(&self, cmd: ast::Command, stdin: Stdio) -> anyhow::Result<Child> {
        match cmd {
            ast::Command::Simple(simple_cmd) => {
                if simple_cmd.len() == 0 {
                    return Err(anyhow!("command is empty"));
                }

                let mut it = simple_cmd.into_iter();
                let cmd_name = it.next().unwrap().0;
                let args = it
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|a| a.clone())
                    .collect();

                match cmd_name.as_str() {
                    // "cd" => self.run_cd_command(&args),
                    // "exit" => self.run_exit_command(&args),
                    _ => self.run_external_command(&cmd_name, &args, stdin),
                }
            },
            ast::Command::Pipeline(a_cmd, b_cmd) => {
                let mut a_cmd_handle = self.eval_command(*a_cmd, stdin)?;
                let b_cmd_handle =
                    self.eval_command(*b_cmd, Stdio::from(a_cmd_handle.stdout.take().unwrap()))?;
                Ok(b_cmd_handle)
            },
        }
    }

    fn run_cd_command(&self, args: &Vec<String>) {
        // if empty default to root (for now)
        let raw_path = if let Some(path) = args.get(0) {
            path
        } else {
            "/"
        };
        let path = Path::new(raw_path);
        if let Err(e) = env::set_current_dir(path) {
            eprintln!("{}", e);
        }
    }

    fn run_exit_command(&self, args: &Vec<String>) {
        std::process::exit(0);
    }

    fn run_external_command(
        &self,
        cmd: &str,
        args: &Vec<String>,
        stdin: Stdio,
    ) -> anyhow::Result<Child> {
        use std::process::Command;

        let child = Command::new(cmd)
            .args(args)
            .stdin(stdin)
            .stdout(Stdio::piped())
            .spawn()?;
        Ok(child)
    }
}
