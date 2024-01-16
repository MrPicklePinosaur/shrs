//! Abstraction for the shell language interpreter
//!
//!

use std::{collections::HashMap, rc::Rc};

use anyhow::anyhow;

use crate::{
    cmd_output::CmdOutput,
    shell::{Context, Runtime, Shell},
};

/// Trait to implement a shell command language
pub trait Lang {
    // TODO make function signature of this MUCH more generic
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> anyhow::Result<CmdOutput>;
    fn name(&self) -> String;
    fn needs_line_check(&self, cmd: String) -> bool;
}

pub struct Langs {
    current_lang: Rc<dyn Lang>,
    lang_map: HashMap<String, Rc<dyn Lang + 'static>>,
}

impl Langs {
    /// Create a new instance of lang
    ///
    /// Must be initialized with at least one language
    pub fn new(name: &str, lang: impl Lang + 'static) -> Self {
        let mut lang_map = HashMap::new();
        lang_map.insert(name.into(), Rc::new(lang) as Rc<dyn Lang>);
        Langs {
            current_lang: lang_map.get(name).unwrap().clone(),
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
}
