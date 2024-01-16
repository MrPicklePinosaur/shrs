mod builtin;
mod interpreter;
mod lang;
mod lang_options;

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use anyhow::anyhow;
use builtin::MuxBuiltin;
use lang::{BashLang, MuxLang, NuLang, PythonLang};
use lang_options::{swap_lang_options, LangOptions};
use shrs::prelude::*;

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
}

impl MuxPlugin {
    pub fn new() -> Self {
        MuxPlugin {
            lang_options: LangOptions::default(),
        }
    }
}

impl Plugin for MuxPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        let mut mux_state = MuxState::new("shrs", PosixLang::default());
        mux_state.register_lang("bash", BashLang::new());
        mux_state.register_lang("nu", NuLang::new());
        mux_state.register_lang("py", PythonLang::new());
        shell.state.insert(mux_state);

        shell.builtins.insert("mux", MuxBuiltin::new());
        shell.lang = Box::new(MuxLang::new());
        shell.hooks.insert(swap_lang_options);

        Ok(())
    }
}
