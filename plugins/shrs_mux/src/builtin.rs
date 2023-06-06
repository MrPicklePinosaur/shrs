use shrs::prelude::*;

use crate::{ChangeLangCtx, MuxState};

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
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<BuiltinStatus> {
        // TODO flag to list all possible languages
        match args.get(0).map(|s| s.as_str()) {
            Some("-l") => {
                ctx.state.get::<MuxState>().map(|state| {
                    println!("Available languages:");
                    for lang in state.registered_langs() {
                        println!("{lang}");
                    }
                });
            },
            Some(lang_name) => {
                if let Some(state) = ctx.state.get_mut::<MuxState>() {
                    let hook_ctx = ChangeLangCtx {
                        old_lang: state.get_lang().to_string(),
                        new_lang: lang_name.to_string(),
                    };
                    match state.set_lang(lang_name) {
                        Ok(_) => println!("setting lang to {lang_name}"),
                        Err(e) => eprintln!("{e}"),
                    }

                    sh.hooks
                        .run(sh, ctx, rt, hook_ctx)
                        .expect("failed running hook");
                };
            },
            _ => return Ok(BuiltinStatus::error()),
        };

        Ok(BuiltinStatus::success())
    }
}
