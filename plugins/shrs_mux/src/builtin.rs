use std::collections::HashSet;

use shrs::prelude::*;

use crate::{lang::NuLang, MuxState};

// TODO make shell mode part of state so we can modify from anywhere?
// TODO add custom hook from when we switch shell mode

#[derive(Default)]
pub struct MuxBuiltin {
    registered_langs: HashSet<String>,
}

impl FromIterator<String> for MuxBuiltin {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        MuxBuiltin {
            registered_langs: HashSet::from_iter(iter),
        }
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
        // TODO hardcoded for now
        // TODO think about how to implement shell switching at runtime (currently running into
        // some ownership issues in shrs/shell.rs)
        match args.get(0).map(|s| s.as_str()) {
            Some(lang_name) => {
                ctx.state.get_mut::<MuxState>().map(|state| {
                    // first check that lang is valid
                    if self.registered_langs.contains(lang_name) {
                        state.lang = lang_name.to_owned().to_string();
                        println!("setting lang to {}", lang_name);
                    } else {
                        eprintln!("invalid lang {}", lang_name);
                    }
                });
            },
            _ => return dummy_child(),
        };

        dummy_child()
    }
}
