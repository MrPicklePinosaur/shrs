use std::{
    env,
    fs::File,
    io::{stdin, stdout, Write},
    os::unix::process::CommandExt,
    path::Path,
    process::{Child, Output, Stdio},
};

use anyhow::anyhow;

use crate::{ast, parser, signal::sig_handler};

/// Default prompt
pub fn simple_prompt() {
    print!("> ");
    stdout().flush();
}
pub fn simple_error() {}

/// Default formmater for displaying the exit code of the previous command
pub fn simple_exit_code(code: i32) {
    println!("[exit +{}]", code);
}

pub type PromptCommand = fn();
pub type ErrorCommand = fn();
pub type ExitCodeCommand = fn(i32);
pub struct Shell {
    /// User defined command that gets ran when we wish to print the prompt
    pub prompt_command: PromptCommand,
    /// User defined command for formatting shell error messages
    pub error_command: ErrorCommand,
    /// User defined command for formatting exit code of previous command
    pub exit_code_command: ExitCodeCommand,
}

impl Shell {
    pub fn run(&mut self) -> anyhow::Result<()> {
        sig_handler()?;

        loop {
            (self.prompt_command)();

            let mut line = String::new();
            if let Err(e) = stdin().read_line(&mut line) {
                continue;
            }

            let mut parser = parser::ParserContext::new();
            match parser.parse(&line) {
                Ok(cmd) => {
                    let cmd_handle =
                        self.eval_command(cmd, Stdio::inherit(), Stdio::piped(), None)?;
                    self.command_output(cmd_handle)?;
                },
                Err(e) => {
                    eprintln!("{}", e);
                },
            }
        }
    }

    // TODO function signature is very ugly
    // TODO maybe make this a method of Command
    pub fn eval_command(
        &mut self,
        cmd: ast::Command,
        stdin: Stdio,
        stdout: Stdio,
        pgid: Option<i32>,
    ) -> anyhow::Result<Child> {
        match cmd {
            ast::Command::Simple { args, redirects } => {
                if args.len() == 0 {
                    return Err(anyhow!("command is empty"));
                }
                // println!("redirects {:?}", redirects);

                // file redirections
                // TODO: current behavior, only one read and write operation is allowed, the latter ones will override the behavior of eariler ones
                let mut cur_stdin = stdin;
                let mut cur_stdout = stdout;
                for redirect in redirects {
                    let filename = Path::new(&*redirect.file);
                    // TODO not making use of file descriptor at all right now
                    let n = match redirect.n {
                        Some(n) => *n,
                        None => 1,
                    };
                    match redirect.mode {
                        ast::RedirectMode::Read => {
                            let file_handle = File::options().read(true).open(filename).unwrap();
                            cur_stdin = Stdio::from(file_handle);
                        },
                        ast::RedirectMode::Write => {
                            let file_handle = File::options()
                                .write(true)
                                .create_new(true)
                                .open(filename)
                                .unwrap();
                            cur_stdout = Stdio::from(file_handle);
                        },
                        ast::RedirectMode::ReadAppend => {
                            let file_handle = File::options()
                                .read(true)
                                .append(true)
                                .open(filename)
                                .unwrap();
                            cur_stdin = Stdio::from(file_handle);
                        },
                        ast::RedirectMode::WriteAppend => {
                            let file_handle = File::options()
                                .write(true)
                                .append(true)
                                .create_new(true)
                                .open(filename)
                                .unwrap();
                            cur_stdout = Stdio::from(file_handle);
                        },
                        ast::RedirectMode::ReadDup => {
                            unimplemented!()
                        },
                        ast::RedirectMode::WriteDup => {
                            unimplemented!()
                        },
                        ast::RedirectMode::ReadWrite => {
                            let file_handle = File::options()
                                .read(true)
                                .write(true)
                                .create_new(true)
                                .open(filename)
                                .unwrap();
                            cur_stdin = Stdio::from(file_handle.try_clone().unwrap());
                            cur_stdout = Stdio::from(file_handle);
                        },
                    };
                }

                let mut it = args.into_iter();
                let cmd_name = it.next().unwrap().0;
                let args = it
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|a| a.clone())
                    .collect();

                // TODO which stdin var to use?, previous command or from file redirection?

                match cmd_name.as_str() {
                    "cd" => self.run_cd_command(&args),
                    "exit" => self.run_exit_command(&args),
                    _ => self.run_external_command(&cmd_name, &args, cur_stdin, cur_stdout, None),
                }
            },
            ast::Command::Pipeline(a_cmd, b_cmd) => {
                // TODO double check that pgid works properly for pipelines that are longer than one pipe (left recursiveness of parser might mess this up)
                let mut a_cmd_handle = self.eval_command(*a_cmd, stdin, Stdio::piped(), None)?;
                let piped_stdin = Stdio::from(a_cmd_handle.stdout.take().unwrap());
                let pgid = a_cmd_handle.id();
                let b_cmd_handle =
                    self.eval_command(*b_cmd, piped_stdin, stdout, Some(pgid as i32))?;
                Ok(b_cmd_handle)
            },
            ast::Command::And(a_cmd, b_cmd) => {
                // TODO double check if these stdin and stdou params are correct
                let a_cmd_handle =
                    self.eval_command(*a_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                if let Some(output) = self.command_output(a_cmd_handle)? {
                    if !output.status.success() {
                        // TODO return something better (indicate that command failed with exit code)
                        return dummy_child();
                    }
                }
                let b_cmd_handle =
                    self.eval_command(*b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                Ok(b_cmd_handle)
            },
            // duplicate of And (could abstract a bit)
            ast::Command::Or(a_cmd, b_cmd) => {
                let a_cmd_handle =
                    self.eval_command(*a_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                if let Some(output) = self.command_output(a_cmd_handle)? {
                    if output.status.success() {
                        return dummy_child();
                    }
                }
                let b_cmd_handle =
                    self.eval_command(*b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                Ok(b_cmd_handle)
            },
            ast::Command::Not(cmd) => {
                // TODO exit status negate
                let cmd_handle = self.eval_command(*cmd, stdin, stdout, None)?;
                Ok(cmd_handle)
            },
            ast::Command::AsyncList(a_cmd, b_cmd) => {
                let a_cmd_handle =
                    self.eval_command(*a_cmd, Stdio::inherit(), Stdio::piped(), None)?;

                match b_cmd {
                    None => Ok(a_cmd_handle),
                    Some(b_cmd) => {
                        let b_cmd_handle =
                            self.eval_command(*b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                        Ok(b_cmd_handle)
                    },
                }
            },
            ast::Command::SeqList(a_cmd, b_cmd) => {
                // TODO very similar to AsyncList
                let a_cmd_handle =
                    self.eval_command(*a_cmd, Stdio::inherit(), Stdio::piped(), None)?;

                match b_cmd {
                    None => Ok(a_cmd_handle),
                    Some(b_cmd) => {
                        self.command_output(a_cmd_handle)?;
                        let b_cmd_handle =
                            self.eval_command(*b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                        Ok(b_cmd_handle)
                    },
                }
            },
        }
    }

    fn run_cd_command(&self, args: &Vec<String>) -> anyhow::Result<Child> {
        // if empty default to root (for now)
        let raw_path = if let Some(path) = args.get(0) {
            path
        } else {
            "/"
        };
        let path = Path::new(raw_path);
        env::set_current_dir(path)?;

        // return a dummy command
        dummy_child()
    }

    fn run_exit_command(&self, args: &Vec<String>) -> ! {
        std::process::exit(0)
    }

    fn run_external_command(
        &self,
        cmd: &str,
        args: &Vec<String>,
        stdin: Stdio,
        stdout: Stdio,
        pgid: Option<i32>,
    ) -> anyhow::Result<Child> {
        use std::process::Command;

        let child = Command::new(cmd)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .process_group(pgid.unwrap_or(0)) // pgid of 0 means use own pid as pgid
            .spawn()?;

        Ok(child)
    }

    /// Small wrapper that outputs command output if exists
    fn command_output(&self, cmd_handle: Child) -> anyhow::Result<Option<Output>> {
        let cmd_output = cmd_handle.wait_with_output()?;
        print!("{}", std::str::from_utf8(&cmd_output.stdout)?);
        stdout().flush()?;
        (self.exit_code_command)(cmd_output.status.code().unwrap());
        Ok(Some(cmd_output))
    }
}

fn dummy_child() -> anyhow::Result<Child> {
    use std::process::Command;
    let cmd = Command::new("true").spawn()?;
    Ok(cmd)
}
