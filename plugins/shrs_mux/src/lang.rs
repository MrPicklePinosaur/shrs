use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::format,
    io::{BufRead, BufReader, Read, Write},
    ops::Add,
    os::unix::process::ExitStatusExt,
    process::{Child, ChildStderr, ChildStdout, Command, ExitStatus, Stdio},
};

use shrs::{
    lang::{Lexer, Token},
    prelude::*,
};

use crate::{
    interpreter::{read_err, read_out},
    MuxState,
};

pub struct MuxLang {}

impl MuxLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for MuxLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> anyhow::Result<CmdOutput> {
        let Some(state) = ctx.state.get::<MuxState>() else {
            return Ok(CmdOutput::error());
        };

        let (lang_name, lang) = state.current_lang();
        lang.eval(sh, ctx, rt, cmd)
    }

    fn name(&self) -> String {
        "mux".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        //TODO check if open quotes or brackets
        // TODO this is super duplicated code

        if let Some(last_char) = cmd.chars().last() {
            if last_char == '\\' {
                return true;
            }
        };

        let mut brackets: Vec<Token> = vec![];

        let lexer = Lexer::new(cmd.as_str());

        for t in lexer {
            if let Ok(token) = t {
                match token.1 {
                    Token::LBRACE => brackets.push(token.1),
                    Token::LPAREN => brackets.push(token.1),
                    Token::RPAREN => {
                        if let Some(bracket) = brackets.last() {
                            if bracket == &Token::LPAREN {
                                brackets.pop();
                            } else {
                                return false;
                            }
                        }
                    },
                    Token::RBRACE => {
                        if let Some(bracket) = brackets.last() {
                            if bracket == &Token::LBRACE {
                                brackets.pop();
                            } else {
                                return false;
                            }
                        }
                    },
                    Token::WORD(w) => {
                        if let Some(c) = w.chars().next() {
                            if c == '\'' {
                                if w.len() == 1 {
                                    return true;
                                }
                                if let Some(e) = w.chars().last() {
                                    return e != '\'';
                                } else {
                                    return true;
                                }
                            }
                            if c == '\"' {
                                if w.len() == 1 {
                                    return true;
                                }

                                if let Some(e) = w.chars().last() {
                                    return e != '\"';
                                } else {
                                    return true;
                                }
                            }
                        }
                    },

                    _ => (),
                }
            }
        }

        !brackets.is_empty()
    }
}

pub struct NuLang {}

impl NuLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for NuLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut handle = Command::new("nu")
            .args(vec!["-c", &cmd])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let output = handle.wait_with_output()?;
        // ctx.out.print(output.stdout);

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "nu".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}

pub struct PythonLang {
    instance: RefCell<Child>,
}

impl PythonLang {
    pub fn new() -> Self {
        // TODO maybe support custom parameters to pass to command
        let instance = Command::new("python3")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start python process");

        Self {
            instance: RefCell::new(instance),
        }
    }
}

impl Lang for PythonLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut instance = self.instance.borrow_mut();
        let stdin = instance.stdin.as_mut().expect("Failed to open stdin");

        stdin
            .write_all((cmd + "\n").as_bytes())
            .expect("Python command failed");

        let stdout = instance.stdout.as_mut().expect("Failed to open stdout");
        let stdout_reader = BufReader::new(stdout);
        read_out(ctx, stdout_reader)?;

        Ok(CmdOutput::success())
    }

    fn name(&self) -> String {
        "python".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}

pub struct BashLang {
    instance: RefCell<Child>,
}

impl BashLang {
    pub fn new() -> Self {
        Self {
            instance: RefCell::new(
                Command::new("bash")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("Failed to start bash process"),
            ),
        }
    }
}

impl Lang for BashLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<CmdOutput> {
        let mut instance = self.instance.borrow_mut();
        let stdin = instance.stdin.as_mut().expect("Failed to open stdin");

        let cd_statement = format!("cd {}\n", rt.working_dir.to_string_lossy());

        stdin
            .write_all(cd_statement.as_bytes())
            .expect("unable to set var");

        for (k, v) in rt.env.iter() {
            let export_statement = format!("export {}={:?}\n", k, v);
            stdin
                .write_all(export_statement.as_bytes())
                .expect("unable to set var");
        }
        stdin
            .write_all((cmd + ";echo $?'\x1A'; echo '\x1A' >&2\n").as_bytes())
            .expect("Bash command failed");

        let stdout_reader =
            BufReader::new(instance.stdout.as_mut().expect("Failed to open stdout"));
        let status = read_out(ctx, stdout_reader)?;

        let stderr_reader =
            BufReader::new(instance.stderr.as_mut().expect("Failed to open stdout"));
        read_err(ctx, stderr_reader)?;

        Ok(CmdOutput::new(status))
    }

    fn name(&self) -> String {
        "bash".to_string()
    }

    fn needs_line_check(&self, cmd: String) -> bool {
        false
    }
}
