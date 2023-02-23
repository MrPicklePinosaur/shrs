use std::{
    env,
    fs::File,
    io::{stdin, stdout, Write},
    os::unix::process::CommandExt,
    path::Path,
    process::{Child, Output, Stdio},
};

use anyhow::anyhow;

use crate::{
    alias::Alias,
    ast::{self, Assign},
    builtin::Builtins,
    env::Env,
    history::History,
    parser,
    signal::sig_handler,
};

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
// TODO ideas for hooks:
//   - exit hook: print message when shell exits (by exit builtin)

pub struct Hooks {
    /// User defined command that gets ran when we wish to print the prompt
    pub prompt_command: PromptCommand,
    /// User defined command for formatting shell error messages
    pub error_command: ErrorCommand,
    /// User defined command for formatting exit code of previous command
    pub exit_code_command: ExitCodeCommand,
}

impl Default for Hooks {
    fn default() -> Self {
        Hooks {
            prompt_command: simple_prompt,
            error_command: simple_error,
            exit_code_command: simple_exit_code,
        }
    }
}

pub struct Shell {
    hooks: Hooks,
    builtins: Builtins,
}

// Runtime context for the shell
pub struct Context {
    pub history: History,
    pub env: Env,
    pub alias: Alias,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            history: History::new(),
            env: Env::new(),
            alias: Alias::new(),
        }
    }
}

impl Shell {
    pub fn new(hooks: Hooks, builtins: Builtins) -> Self {
        Shell { hooks, builtins }
    }

    pub fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        // init stuff
        sig_handler()?;
        ctx.env.load();

        loop {
            (self.hooks.prompt_command)();

            let mut line = String::new();
            if let Err(e) = stdin().read_line(&mut line) {
                continue;
            }
            line = line.trim_end().to_string();

            // strip newline

            // attempt to expand alias
            let expanded = ctx.alias.get(&line).unwrap_or(&line);

            // wether the command pre-alias expansion or post should be added to history could be a configuration option
            ctx.history.add(expanded.clone());

            // TODO rewrite the error handling here better
            let mut parser = parser::ParserContext::new();
            let cmd = match parser.parse(&expanded) {
                Ok(cmd) => cmd,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                },
            };
            let cmd_handle =
                match self.eval_command(ctx, cmd, Stdio::inherit(), Stdio::piped(), None) {
                    Ok(cmd_handle) => cmd_handle,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    },
                };
            self.command_output(cmd_handle)?;
        }
    }

    // TODO function signature is very ugly
    // TODO maybe make this a method of Command
    pub fn eval_command(
        &self,
        ctx: &mut Context,
        cmd: ast::Command,
        stdin: Stdio,
        stdout: Stdio,
        pgid: Option<i32>,
    ) -> anyhow::Result<Child> {
        match cmd {
            ast::Command::Simple {
                assigns,
                args,
                redirects,
            } => {
                if args.len() == 0 {
                    return Err(anyhow!("command is empty"));
                }
                // println!("redirects {:?}", redirects);
                // println!("assigns {:?}", assigns);

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

                // TODO currently don't support assignment for builtins (should it be supported even?)
                match cmd_name.as_str() {
                    "cd" => self.builtins.cd.run(ctx, &args),
                    "exit" => self.builtins.exit.run(ctx, &args),
                    "history" => self.builtins.history.run(ctx, &args),
                    "debug" => self.builtins.debug.run(ctx, &args),
                    _ => self.run_external_command(
                        &cmd_name, &args, cur_stdin, cur_stdout, None, assigns,
                    ),
                }
            },
            ast::Command::Pipeline(a_cmd, b_cmd) => {
                // TODO double check that pgid works properly for pipelines that are longer than one pipe (left recursiveness of parser might mess this up)
                let mut a_cmd_handle =
                    self.eval_command(ctx, *a_cmd, stdin, Stdio::piped(), None)?;
                let piped_stdin = Stdio::from(a_cmd_handle.stdout.take().unwrap());
                let pgid = a_cmd_handle.id();
                let b_cmd_handle =
                    self.eval_command(ctx, *b_cmd, piped_stdin, stdout, Some(pgid as i32))?;
                Ok(b_cmd_handle)
            },
            ast::Command::And(a_cmd, b_cmd) => {
                // TODO double check if these stdin and stdou params are correct
                let a_cmd_handle =
                    self.eval_command(ctx, *a_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                if let Some(output) = self.command_output(a_cmd_handle)? {
                    if !output.status.success() {
                        // TODO return something better (indicate that command failed with exit code)
                        return dummy_child();
                    }
                }
                let b_cmd_handle =
                    self.eval_command(ctx, *b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                Ok(b_cmd_handle)
            },
            // duplicate of And (could abstract a bit)
            ast::Command::Or(a_cmd, b_cmd) => {
                let a_cmd_handle =
                    self.eval_command(ctx, *a_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                if let Some(output) = self.command_output(a_cmd_handle)? {
                    if output.status.success() {
                        return dummy_child();
                    }
                }
                let b_cmd_handle =
                    self.eval_command(ctx, *b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                Ok(b_cmd_handle)
            },
            ast::Command::Not(cmd) => {
                // TODO exit status negate
                let cmd_handle = self.eval_command(ctx, *cmd, stdin, stdout, None)?;
                Ok(cmd_handle)
            },
            ast::Command::AsyncList(a_cmd, b_cmd) => {
                let a_cmd_handle =
                    self.eval_command(ctx, *a_cmd, Stdio::inherit(), Stdio::piped(), None)?;

                match b_cmd {
                    None => Ok(a_cmd_handle),
                    Some(b_cmd) => {
                        let b_cmd_handle =
                            self.eval_command(ctx, *b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                        Ok(b_cmd_handle)
                    },
                }
            },
            ast::Command::SeqList(a_cmd, b_cmd) => {
                // TODO very similar to AsyncList
                let a_cmd_handle =
                    self.eval_command(ctx, *a_cmd, Stdio::inherit(), Stdio::piped(), None)?;

                match b_cmd {
                    None => Ok(a_cmd_handle),
                    Some(b_cmd) => {
                        self.command_output(a_cmd_handle)?;
                        let b_cmd_handle =
                            self.eval_command(ctx, *b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                        Ok(b_cmd_handle)
                    },
                }
            },
        }
    }

    fn run_external_command(
        &self,
        cmd: &str,
        args: &Vec<String>,
        stdin: Stdio,
        stdout: Stdio,
        pgid: Option<i32>,
        assigns: Vec<Assign>,
    ) -> anyhow::Result<Child> {
        use std::process::Command;

        let envs = assigns.iter().map(|word| (&word.var.0, &word.val.0));

        let child = Command::new(cmd)
            .args(args)
            .stdin(stdin)
            .stdout(stdout)
            .process_group(pgid.unwrap_or(0)) // pgid of 0 means use own pid as pgid
            .envs(envs)
            .spawn()?;

        Ok(child)
    }

    /// Small wrapper that outputs command output if exists
    fn command_output(&self, cmd_handle: Child) -> anyhow::Result<Option<Output>> {
        let cmd_output = cmd_handle.wait_with_output()?;
        print!("{}", std::str::from_utf8(&cmd_output.stdout)?);
        stdout().flush()?;
        (self.hooks.exit_code_command)(cmd_output.status.code().unwrap());
        Ok(Some(cmd_output))
    }
}

pub fn dummy_child() -> anyhow::Result<Child> {
    use std::process::Command;
    let cmd = Command::new("true").spawn()?;
    Ok(cmd)
}
