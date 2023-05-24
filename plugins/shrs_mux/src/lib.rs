mod builtin;
mod lang;

use std::collections::HashMap;

use builtin::MuxBuiltin;
use lang::{MuxLang, NuLang, PythonLang};
use shrs::prelude::*;

struct MuxState {
    // TODO I don't very like this 'string'-typing
    pub lang: String,
}

impl Default for MuxState {
    fn default() -> Self {
        MuxState {
            lang: String::from("shrs"),
        }
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
        let langs: HashMap<String, Box<dyn Lang>> = HashMap::from_iter(vec![
            (
                "shrs".into(),
                Box::new(PosixLang::default()) as Box<dyn Lang>,
            ),
            ("nu".into(), Box::new(NuLang::new()) as Box<dyn Lang>),
            ("py".into(), Box::new(PythonLang::new()) as Box<dyn Lang>),
        ]);

        shell.builtins.insert(
            "mux",
            MuxBuiltin::from_iter(langs.iter().map(|(lang_name, _)| lang_name.to_owned())),
        );
        shell.lang = Box::new(MuxLang::new(langs));
        shell.state.insert(MuxState::default())
    }
}
