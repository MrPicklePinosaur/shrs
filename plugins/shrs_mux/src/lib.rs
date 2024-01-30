mod builtin;
mod interpreter;
mod lang;
mod lang_options;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use anyhow::anyhow;
use builtin::MuxBuiltin;
pub use lang::{BashLang, NuLang, PythonLang, SshLang};
use lang_options::swap_lang_options;
pub use lang_options::LangOptions;
use shrs::{
    lang::{Lexer, Token},
    prelude::*,
};

use crate::interpreter::{read_err, read_out};

pub struct MuxState {
    current_lang: (String, Rc<dyn Lang>),
    lang_map: HashMap<String, Rc<dyn Lang + 'static>>,
}

impl MuxState {
    /// Create a new instance of lang
    ///
    /// Must be initialized with at least one language
    pub fn new(name: &str, lang: impl Lang + 'static) -> Self {
        let mut lang_map = HashMap::new();
        lang_map.insert(name.into(), Rc::new(lang) as Rc<dyn Lang>);
        Self {
            current_lang: (name.into(), lang_map.get(name).unwrap().clone()),
            lang_map,
        }
    }

    /// Register a language using a name
    ///
    /// If the language has already been registered previously, it is overwritten
    pub fn register_lang(&mut self, name: &str, lang: impl Lang + 'static) {
        self.lang_map.insert(name.into(), Rc::new(lang));
    }

    /// Get the current language
    pub fn current_lang(&self) -> (String, Rc<dyn Lang>) {
        self.current_lang.clone()
    }

    /// Set the language using the name
    ///
    /// Selecting invalid language returns error
    pub fn set_current_lang(&mut self, name: &str) -> anyhow::Result<()> {
        if let Some(lang) = self.lang_map.get(name) {
            self.current_lang = (name.into(), lang.clone());
        } else {
            return Err(anyhow!("Invalid language: {}", name));
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Rc<dyn Lang>)> {
        self.lang_map.iter()
    }
}

#[derive(Clone)]
/// Hook that emitted when the language is changed
pub struct ChangeLangCtx {
    old_lang: String,
    new_lang: String,
}

pub struct MuxPlugin {
    lang_options: LangOptions,
    // TODO kinda stupid but need to pass ownership to state
    mux_state: RefCell<Option<MuxState>>,
}

impl MuxPlugin {
    pub fn new() -> Self {
        let mux_state = MuxState::new("shrs", PosixLang::default());

        MuxPlugin {
            lang_options: LangOptions::default(),
            mux_state: RefCell::new(Some(mux_state)),
        }
    }

    // Proxy to register_lang of underlying MuxState
    pub fn register_lang(self, name: &str, lang: impl Lang + 'static) -> Self {
        // TODO make sure not called after plugin is inited
        self.mux_state
            .borrow_mut()
            .as_mut()
            .unwrap()
            .register_lang(name, lang);
        self
    }

    // TODO maybe add abilitiy to set the default lang
}

impl Plugin for MuxPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        // TODO pretty bad solution to just clone everything
        let mut mux_state_borrow = self.mux_state.borrow_mut();
        let mux_state = mux_state_borrow.take().unwrap();
        shell.state.insert(mux_state);

        shell.builtins.insert("mux", MuxBuiltin::new());
        shell.lang = Box::new(MuxLang::new());
        shell.hooks.insert(swap_lang_options);

        Ok(())
    }
}

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
