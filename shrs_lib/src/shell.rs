use std::{
    env,
    fs::File,
    io::{stdin, stdout, Write},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Output, Stdio},
};

use anyhow::anyhow;
use reedline::{History, HistoryItem};

use crate::{
    alias::Alias,
    ast::{self, Assign},
    builtin::Builtins,
    env::Env,
    history::MemHistory,
    hooks::{Hooks, StartupHookCtx},
    lexer::Lexer,
    parser,
    prompt::CustomPrompt,
    signal::sig_handler,
};

#[derive(Default)]
pub struct Shell {
    pub hooks: Hooks,
    pub builtins: Builtins,
    pub prompt: CustomPrompt,
}

// (shared) shell context
pub struct Context {
    pub history: Box<dyn History>,
    pub alias: Alias,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            history: Box::new(MemHistory::new()),
            alias: Alias::new(),
        }
    }
}

// Runtime context for the shell
#[derive(Clone)]
pub struct Runtime {
    pub working_dir: PathBuf,
    pub env: Env,
    /// Name of the shell or shell script
    pub name: String,
    /// Arguments this shell was called with
    pub args: Vec<String>,
    /// Exit status of most recent pipeline
    pub exit_status: i32,
}

impl Default for Runtime {
    fn default() -> Self {
        Runtime {
            env: Env::new(),
            working_dir: std::env::current_dir().unwrap(),
            // TODO currently hardcoded
            name: "shrs".into(),
            // TDOO currently unused (since we have not implemented functions etc)
            args: vec![],
            exit_status: 0,
        }
    }
}

impl Shell {
    pub fn run(&self, ctx: &mut Context, rt: &mut Runtime) -> anyhow::Result<()> {
        use reedline::*;

        // init stuff
        sig_handler()?;
        rt.env.load();

        // for now complete command names only
        let completions = find_executables_in_path(rt.env.get("PATH").unwrap());

        let completer = Box::new(DefaultCompleter::new_with_wordlen(completions, 2));
        let completion_menu = Box::new(ColumnarMenu::default().with_name("completion_menu"));

        let mut insert_bindings = default_vi_insert_keybindings();
        insert_bindings.add_binding(
            KeyModifiers::NONE,
            KeyCode::Tab,
            ReedlineEvent::UntilFound(vec![
                ReedlineEvent::Menu("completion_menu".to_string()),
                ReedlineEvent::MenuNext,
            ]),
        );

        let normal_bindings = default_vi_normal_keybindings();

        let mut line_editor = Reedline::create()
            .with_edit_mode(Box::new(Vi::new(insert_bindings, normal_bindings)))
            .with_completer(completer)
            .with_menu(ReedlineMenu::EngineCompleter(completion_menu));

        let prompt = shrs_line::prompt::DefaultPrompt::new();
        let readline = shrs_line::Line::new();

        (self.hooks.startup)(StartupHookCtx { startup_time: 0 });

        loop {
            let line = readline.read_line(&prompt);

            // attempt to expand alias
            let expanded = ctx.alias.get(&line).unwrap_or(&line).clone();

            // TODO rewrite the error handling here better
            let lexer = Lexer::new(&expanded);
            let mut parser = parser::ParserContext::new();
            let cmd = match parser.parse(lexer) {
                Ok(cmd) => cmd,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                },
            };
            let cmd_handle =
                match self.eval_command(ctx, rt, &cmd, Stdio::inherit(), Stdio::piped(), None) {
                    Ok(cmd_handle) => cmd_handle,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    },
                };
            self.command_output(rt, cmd_handle)?;
        }
    }

    // TODO function signature is very ugly
    // TODO maybe make this a method of Command
    pub fn eval_command(
        &self,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: &ast::Command,
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
                    let n = match &redirect.n {
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
                let cmd_name = &it.next().unwrap();
                let args = it
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|a| (*a).clone())
                    .collect();

                // TODO which stdin var to use?, previous command or from file redirection?

                // TODO currently don't support assignment for builtins (should it be supported even?)
                match cmd_name.as_str() {
                    "cd" => self.builtins.cd.run(ctx, rt, &args),
                    "exit" => self.builtins.exit.run(ctx, rt, &args),
                    "history" => self.builtins.history.run(ctx, rt, &args),
                    "debug" => self.builtins.debug.run(ctx, rt, &args),
                    _ => self.run_external_command(
                        ctx, rt, &cmd_name, &args, cur_stdin, cur_stdout, None, assigns,
                    ),
                }
            },
            ast::Command::Pipeline(a_cmd, b_cmd) => {
                // TODO double check that pgid works properly for pipelines that are longer than one pipe (left recursiveness of parser might mess this up)
                let mut a_cmd_handle =
                    self.eval_command(ctx, rt, a_cmd, stdin, Stdio::piped(), None)?;
                let piped_stdin = Stdio::from(a_cmd_handle.stdout.take().unwrap());
                let pgid = a_cmd_handle.id();
                let b_cmd_handle =
                    self.eval_command(ctx, rt, b_cmd, piped_stdin, stdout, Some(pgid as i32))?;
                Ok(b_cmd_handle)
            },
            ast::Command::Or(a_cmd, b_cmd) | ast::Command::And(a_cmd, b_cmd) => {
                let negate = match cmd {
                    ast::Command::Or { .. } => false,
                    ast::Command::And { .. } => true,
                    _ => unreachable!(),
                };
                // TODO double check if these stdin and stdou params are correct
                let a_cmd_handle =
                    self.eval_command(ctx, rt, a_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                if let Some(output) = self.command_output(rt, a_cmd_handle)? {
                    if output.status.success() ^ negate {
                        // TODO return something better (indicate that command failed with exit code)
                        return dummy_child();
                    }
                }
                let b_cmd_handle =
                    self.eval_command(ctx, rt, b_cmd, Stdio::inherit(), Stdio::piped(), None)?;
                Ok(b_cmd_handle)
            },
            ast::Command::Not(cmd) => {
                // TODO exit status negate
                let cmd_handle = self.eval_command(ctx, rt, cmd, stdin, stdout, None)?;
                Ok(cmd_handle)
            },
            ast::Command::AsyncList(a_cmd, b_cmd) => {
                let a_cmd_handle =
                    self.eval_command(ctx, rt, a_cmd, Stdio::inherit(), Stdio::piped(), None)?;

                match b_cmd {
                    None => Ok(a_cmd_handle),
                    Some(b_cmd) => {
                        let b_cmd_handle = self.eval_command(
                            ctx,
                            rt,
                            b_cmd,
                            Stdio::inherit(),
                            Stdio::piped(),
                            None,
                        )?;
                        Ok(b_cmd_handle)
                    },
                }
            },
            ast::Command::SeqList(a_cmd, b_cmd) => {
                // TODO very similar to AsyncList
                let a_cmd_handle =
                    self.eval_command(ctx, rt, a_cmd, Stdio::inherit(), Stdio::piped(), None)?;

                match b_cmd {
                    None => Ok(a_cmd_handle),
                    Some(b_cmd) => {
                        self.command_output(rt, a_cmd_handle)?;
                        let b_cmd_handle = self.eval_command(
                            ctx,
                            rt,
                            b_cmd,
                            Stdio::inherit(),
                            Stdio::piped(),
                            None,
                        )?;
                        Ok(b_cmd_handle)
                    },
                }
            },
            ast::Command::Subshell(cmd) => {
                // TODO rn history is being copied too, history (and also alias?) really should be global
                // maybe seperate out global context and runtime context into two structs?
                let mut new_rt = rt.clone();
                let cmd_handle = self.eval_command(
                    ctx,
                    &mut new_rt,
                    cmd,
                    Stdio::inherit(),
                    Stdio::piped(),
                    None,
                )?;
                Ok(cmd_handle)
            },
            ast::Command::If { conds, else_part } => {
                // TODO throw proper error here
                assert!(conds.len() >= 1);

                for ast::Condition { cond, body } in conds {
                    let cond_handle =
                        self.eval_command(ctx, rt, cond, Stdio::inherit(), Stdio::piped(), None)?;
                    // TODO sorta similar to and statements
                    if let Some(output) = self.command_output(rt, cond_handle)? {
                        if output.status.success() {
                            let body_handle = self.eval_command(
                                ctx,
                                rt,
                                body,
                                Stdio::inherit(),
                                Stdio::piped(),
                                None,
                            )?;
                            return Ok(body_handle);
                        }
                    }
                }

                if let Some(else_part) = else_part {
                    let else_handle = self.eval_command(
                        ctx,
                        rt,
                        else_part,
                        Stdio::inherit(),
                        Stdio::piped(),
                        None,
                    )?;
                    return Ok(else_handle);
                }

                dummy_child()
            },
            ast::Command::While { cond, body } | ast::Command::Until { cond, body } => {
                let negate = match cmd {
                    ast::Command::While { .. } => false,
                    ast::Command::Until { .. } => true,
                    _ => unreachable!(),
                };

                loop {
                    let cond_handle =
                        self.eval_command(ctx, rt, cond, Stdio::inherit(), Stdio::piped(), None)?;
                    // TODO sorta similar to if statements
                    if let Some(output) = self.command_output(rt, cond_handle)? {
                        if output.status.success() ^ negate {
                            let body_handle = self.eval_command(
                                ctx,
                                rt,
                                body,
                                Stdio::inherit(),
                                Stdio::piped(),
                                None,
                            )?;
                            self.command_output(rt, body_handle)?;
                        } else {
                            break;
                        }
                    } else {
                        break; // TODO not sure if there should be break here
                    }
                }
                dummy_child()
            },
            ast::Command::For {
                name,
                wordlist,
                body,
            } => {
                // expand wordlist
                let mut expanded = vec![];
                for word in wordlist {
                    // TODO use IFS variable for this
                    for subword in word.split(" ") {
                        expanded.push(subword);
                    }
                }

                // execute body
                for word in expanded {
                    // TODO should have seperate variable struct instead of env
                    rt.env.set(name, word); // TODO unset the var after the loop?
                    let body_handle =
                        self.eval_command(ctx, rt, body, Stdio::inherit(), Stdio::piped(), None)?;
                    self.command_output(rt, body_handle)?;
                }

                dummy_child()
            },
            ast::Command::Case { word, arms } => {
                // println!("word {:?}, arms {:?}", word, arms);

                let subst_word = envsubst(rt, word);

                for ast::CaseArm { pattern, body } in arms {
                    if pattern.iter().any(|x| x == &subst_word) {
                        let body_handle = self.eval_command(
                            ctx,
                            rt,
                            body,
                            Stdio::inherit(),
                            Stdio::piped(),
                            None,
                        )?;
                        self.command_output(rt, body_handle)?;
                        // TODO should we break? (should multiple match arms be matched?)
                    }
                }

                dummy_child()
            },
            ast::Command::None => dummy_child(),
        }
    }

    fn run_external_command(
        &self,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: &str,
        args: &Vec<String>,
        stdin: Stdio,
        stdout: Stdio,
        pgid: Option<i32>,
        assigns: &Vec<Assign>,
    ) -> anyhow::Result<Child> {
        use std::process::Command;

        let envs = assigns.iter().map(|word| (&word.var, &word.val));

        let subst_args = args.iter().map(|x| envsubst(rt, x)).collect::<Vec<_>>();

        // TODO might need to do subst on cmd too
        let child = Command::new(cmd)
            .args(subst_args)
            .stdin(stdin)
            .stdout(stdout)
            .process_group(pgid.unwrap_or(0)) // pgid of 0 means use own pid as pgid
            .current_dir(rt.working_dir.to_str().unwrap())
            .envs(envs)
            .spawn()?;

        Ok(child)
    }

    /// Small wrapper that outputs command output if exists
    fn command_output(
        &self,
        rt: &mut Runtime,
        cmd_handle: Child,
    ) -> anyhow::Result<Option<Output>> {
        let cmd_output = cmd_handle.wait_with_output()?;
        print!("{}", std::str::from_utf8(&cmd_output.stdout)?);
        stdout().flush()?;
        let exit_code = cmd_output.status.code().unwrap();
        rt.exit_status = exit_code;
        (self.hooks.exit_code)(exit_code);
        Ok(Some(cmd_output))
    }
}

pub fn dummy_child() -> anyhow::Result<Child> {
    use std::process::Command;
    let cmd = Command::new("true").spawn()?;
    Ok(cmd)
}

/// Performs environment substition on a string
// TODO regex replace might not be the best way. could also recognize the env var during parsing
// TODO handle escaped characters
fn envsubst(rt: &mut Runtime, arg: &str) -> String {
    use regex::Regex;

    let mut subst = arg.to_string();

    // substitute special parameters first
    subst = subst.as_str().replace("$?", &rt.exit_status.to_string());
    subst = subst.as_str().replace("$#", &rt.args.len().to_string());
    subst = subst.as_str().replace("$0", &rt.name);

    // TODO precompile regex in lazy_static
    let r_0 = Regex::new(r"\$(?P<env>[a-zA-Z_]+)").unwrap(); // no braces
    let r_1 = Regex::new(r"\$\{(?P<env>[a-zA-Z_]+)\}").unwrap(); // with braces

    for cap in r_0.captures_iter(arg) {
        // look up env var
        let var = &cap["env"];
        // TODO stupid code
        let val = match rt.env.get(var) {
            Some(val) => val.clone(),
            None => String::new(),
        };
        let fmt_env = format!("${}", var); // format $VAR
        subst = subst.as_str().replace(&fmt_env, &val);
    }

    // TODO this is dumb stupid and bad repeated code
    for cap in r_1.captures_iter(arg) {
        let var = &cap["env"];
        let val = match rt.env.get(var) {
            Some(val) => val.clone(),
            None => String::new(),
        };
        let fmt_env = format!("${{{}}}", var); // format ${VAR}
        subst = subst.as_str().replace(&fmt_env, &val);
    }
    subst
}

/// Looks through each directory in path and finds executables
fn find_executables_in_path(path_str: &str) -> Vec<String> {
    use std::{fs, os::unix::fs::PermissionsExt};

    let mut execs = vec![];
    for path in path_str.split(":") {
        let dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => continue,
        };
        for file in dir {
            if let Ok(dir_entry) = file {
                // check if file is executable
                if dir_entry.metadata().unwrap().permissions().mode() & 0o111 != 0 {
                    execs.push(dir_entry.file_name().to_str().unwrap().into());
                }
            }
        }
    }
    execs
}

#[cfg(test)]
mod tests {
    use super::{envsubst, Runtime};
    use crate::shell::find_executables_in_path;

    #[test]
    fn envsubst_test() {
        let mut rt = Runtime::default();
        rt.env.set("EDITOR", "vim");
        rt.env.set("SHELL", "/bin/shrs");
        let text = "$SHELL ${EDITOR}";
        let subst = envsubst(&mut rt, text);
        assert_eq!(subst, String::from("/bin/shrs vim"));
    }

    #[test]
    fn path_execs_test() {
        println!("{:?}", find_executables_in_path("/usr/bin:/usr/local/bin"));
    }
}
