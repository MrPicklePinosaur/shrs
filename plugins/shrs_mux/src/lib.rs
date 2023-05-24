mod builtin;
mod lang;

use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use builtin::MuxBuiltin;
use lang::{MuxLang, NuLang, PythonLang};
use shrs::prelude::*;

pub struct MuxState {
    // TODO I don't very like this 'string'-typing
    lang: String,
    registered_langs: HashSet<String>,
}

impl MuxState {
    /// Construct a new container for keeping track of the currently used shell language
    ///
    /// At least one language must be supplied. The first langauge that is supplied is used as the
    /// starting language
    pub fn new(langs: Vec<String>) -> anyhow::Result<MuxState> {
        let first_lang = match langs.get(0) {
            Some(first_lang) => first_lang,
            None => return Err(anyhow!("require at least one langauge")),
        };

        let res = MuxState {
            lang: first_lang.to_owned(),
            registered_langs: HashSet::from_iter(langs.into_iter()),
        };
        Ok(res)
    }

    /// Set the current language being used by the MuxLang
    ///
    /// If an invalid language is used an error is returned
    pub fn set_lang(&mut self, lang: &str) -> anyhow::Result<()> {
        if self.registered_langs.contains(lang) {
            self.lang = lang.to_owned().to_string();
            Ok(())
        } else {
            Err(anyhow!("invalid lang"))
        }
    }

    /// Get the currently used langauge
    pub fn get_lang(&self) -> &str {
        &self.lang
    }

    /// Get an iterator for list of all the avaliable langauges
    pub fn registered_langs(&self) -> impl Iterator<Item = &String> {
        self.registered_langs.iter()
    }
}

pub struct MuxPlugin;

impl MuxPlugin {
    pub fn new() -> Self {
        MuxPlugin
    }
}

impl Plugin for MuxPlugin {
    fn init(&self, shell: &mut ShellConfig) {
        // This might be able to be indexed by typeid?
        let langs: Vec<(String, Box<dyn Lang>)> = vec![
            (
                "shrs".into(),
                Box::new(PosixLang::default()) as Box<dyn Lang>,
            ),
            ("nu".into(), Box::new(NuLang::new()) as Box<dyn Lang>),
            ("py".into(), Box::new(PythonLang::new()) as Box<dyn Lang>),
        ];

        shell.builtins.insert("mux", MuxBuiltin::new());
        let lang_names = langs
            .iter()
            .map(|(lang_name, _)| lang_name.to_owned())
            .collect::<Vec<_>>();
        shell.state.insert(MuxState::new(lang_names).unwrap());
        let langs_map = HashMap::from_iter(langs);
        shell.lang = Box::new(MuxLang::new(langs_map));
    }
}
