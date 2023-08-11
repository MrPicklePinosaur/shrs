use clap::Parser;
use shrs::prelude::*;

use crate::{ChangeLangCtx, MuxState};

// TODO make shell mode part of state so we can modify from anywhere?
// TODO add custom hook from when we switch shell mode

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    list: bool,
    lang: Option<String>,
}

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
        let cli = Cli::try_parse_from(args)?;

        if cli.list {
            ctx.state.get::<MuxState>().map(|state| {
                println!("Available languages:");
                for lang in state.registered_langs() {
                    println!("{lang}");
                }
            });
        }

        if let Some(lang_name) = cli.lang {
            if let Some(state) = ctx.state.get_mut::<MuxState>() {
                let hook_ctx = ChangeLangCtx {
                    old_lang: state.get_lang().to_string(),
                    new_lang: lang_name.to_string(),
                };
                match state.set_lang(&lang_name) {
                    Ok(_) => println!("setting lang to {lang_name}"),
                    Err(e) => eprintln!("{e}"),
                }

                sh.hooks
                    .run(sh, ctx, rt, hook_ctx)
                    .expect("failed running hook");
            };
        }

        Ok(BuiltinStatus::success())
    }
}
