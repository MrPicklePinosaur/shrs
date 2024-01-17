use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::format,
    io::{BufRead, BufReader, Read, Write},
    ops::Add,
    os::unix::process::ExitStatusExt,
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Stdio},
    sync::Arc,
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
