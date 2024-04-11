mod builtin;
mod highlighter;
mod interpreter;
mod lang;
pub mod python;

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use anyhow::anyhow;
use builtin::MuxBuiltin;
pub use lang::{BashLang, NuLang, SqliteLang, SshLang};

pub use highlighter::MuxHighlighter;
use shrs::{prelude::*, readline::highlight::ShrsSyntaxTheme};

pub struct MuxState {
    current_lang: Rc<dyn Lang>,
    lang_map: HashMap<String, Rc<dyn Lang + 'static>>,
    syntax_themes: HashMap<String, Box<dyn SyntaxTheme>>,
}

impl MuxState {
    /// Create a new instance of lang
    ///
    /// Must be initialized with at least one language
    pub fn new(name: &str, lang: impl Lang + 'static, syntax_theme: impl SyntaxTheme) -> Self {
        let mut lang_map = HashMap::new();
        lang_map.insert(name.into(), Rc::new(lang) as Rc<dyn Lang>);
        Self {
            current_lang: lang_map.get(name).unwrap().clone(),
            syntax_themes: HashMap::new(),
            lang_map,
        }
    }

    /// Register a language using a name
    ///
    /// If the language has already been registered previously, it is overwritten
    pub fn register_lang(&mut self, name: &str, lang: impl Lang + 'static) {
        self.lang_map.insert(name.into(), Rc::new(lang));
    }
    pub fn register_theme(&mut self, name: &str, syntax_theme: Box<dyn SyntaxTheme>) {
        self.syntax_themes.insert(name.into(), syntax_theme);
    }

    /// Get the current language
    pub fn current_lang(&self) -> Rc<dyn Lang> {
        self.current_lang.clone()
    }

    /// Set the language using the name
    ///
    /// Selecting invalid language returns error
    pub fn set_current_lang(&mut self, name: &str) -> anyhow::Result<()> {
        if let Some(lang) = self.lang_map.get(name) {
            self.current_lang = lang.clone();
        } else {
            return Err(anyhow!("Invalid language: {}", name));
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Rc<dyn Lang>)> {
        self.lang_map.iter()
    }

    fn get_syntax_theme(&self) -> Option<&Box<dyn SyntaxTheme>> {
        self.syntax_themes.get(&self.current_lang.name())
    }
}

#[derive(Clone)]
/// Hook that emitted when the language is changed
pub struct ChangeLangCtx {
    old_lang: String,
    new_lang: String,
}

pub struct MuxPlugin {
    // TODO kinda stupid but need to pass ownership to state
    mux_state: RefCell<Option<MuxState>>,
}

impl MuxPlugin {
    pub fn new() -> Self {
        let mux_state = MuxState::new("shrs", PosixLang::default(), ShrsSyntaxTheme::default());

        MuxPlugin {
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
    pub fn register_theme(self, name: &str, syntax_theme: Box<dyn SyntaxTheme>) -> Self {
        self.mux_state
            .borrow_mut()
            .as_mut()
            .unwrap()
            .register_theme(name, syntax_theme);
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

        state.current_lang().eval(sh, ctx, rt, cmd)
    }

    fn name(&self) -> String {
        "mux".to_string()
    }

    fn needs_line_check(&self, state: &LineStateBundle) -> bool {
        let Some(mux_state) = state.ctx.state.get::<MuxState>() else {
            return false;
        };
        let lang = mux_state.current_lang();
        lang.needs_line_check(state)
    }
}
