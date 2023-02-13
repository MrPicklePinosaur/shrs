use std::{
    env,
    fs::File,
    io::{stdin, stdout, Write},
    os::fd::AsRawFd,
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

const FD_TABLE_SIZE: usize = 128;
pub struct Shell {
    // TODO not bounds checked currently
    fd_table: [i32; FD_TABLE_SIZE],
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            fd_table: [-1; FD_TABLE_SIZE],
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
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

    fn eval_command(&mut self, cmd: ast::Command, stdin: Stdio) -> anyhow::Result<Child> {
        match cmd {
            ast::Command::Simple { args, redirects } => {
                if args.len() == 0 {
                    return Err(anyhow!("command is empty"));
                }
                println!("redirects {:?}", redirects);

                // file redirections
                for redirect in redirects {
                    let filename = Path::new(&*redirect.file);
                    // TODO might need to change the default
                    let n = match redirect.n {
                        Some(n) => *n,
                        None => 0,
                    };
                    let file_handle = match redirect.mode {
                        ast::RedirectMode::Read => {
                            File::options().read(true).open(filename).unwrap()
                        },
                        _ => unimplemented!(),
                    };
                    self.fd_table[n] = file_handle.as_raw_fd();
                }

                let mut it = args.into_iter();
                let cmd_name = it.next().unwrap().0;
                let args = it
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|a| a.clone())
                    .collect();

                // SWAP which stdin var to use?, previous command or from file redirection?

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
