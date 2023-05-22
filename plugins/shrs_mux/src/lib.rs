mod builtin;
mod lang;

use builtin::MuxBuiltin;
use lang::NuLang;
use shrs::prelude::*;

pub struct MuxPlugin;

impl Plugin for MuxPlugin {
    fn init(&self, shell: &mut ShellConfig) {
        // shell.lang = Box::new(NuLang);
        shell.builtins.insert("mux", MuxBuiltin::new());
    }
}
