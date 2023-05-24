use std::collections::HashSet;

use shrs::prelude::*;

use crate::{lang::NuLang, MuxState};

// TODO make shell mode part of state so we can modify from anywhere?
// TODO add custom hook from when we switch shell mode

#[derive(Default)]
pub struct MuxBuiltin {}

impl MuxBuiltin {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCmd for MuxBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        _rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<std::process::Child> {
        // TODO flag to list all possible languages
        match args.get(0).map(|s| s.as_str()) {
            Some("-l") => {
                ctx.state.get::<MuxState>().map(|state| {
                    println!("Available languages:");
                    for lang in state.registered_langs() {
                        println!("{}", lang);
                    }
                });
            },
            Some(lang_name) => {
                ctx.state
                    .get_mut::<MuxState>()
                    .map(|state| match state.set_lang(lang_name) {
                        Ok(_) => println!("setting lang to {}", lang_name),
                        Err(e) => eprintln!("{}", e),
                    });
            },
            _ => return dummy_child(),
        };

        dummy_child()
    }
}
