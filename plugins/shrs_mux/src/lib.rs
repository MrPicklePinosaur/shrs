mod builtin;
mod lang;

use builtin::MuxBuiltin;
use lang::{MuxLang, NuLang};
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

impl Plugin for MuxPlugin {
    fn init(&self, shell: &mut ShellConfig) {
        shell.lang = Box::new(MuxLang::new());
        shell.builtins.insert("mux", MuxBuiltin::new());
        shell.state.insert(MuxState::default())
    }
}
