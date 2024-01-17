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
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        let Some(state) = ctx.state.get_mut::<MuxState>() else {
            return Ok(CmdOutput::error());
        };

        if cli.list {
            for (lang_name, _) in state.iter() {
                println!("{lang_name}")
            }
        }

        if let Some(lang_name) = cli.lang {
            let (old_lang_name, _) = state.current_lang();
            let hook_ctx = ChangeLangCtx {
                old_lang: old_lang_name,
                new_lang: lang_name.clone().into(),
            };

            match state.set_current_lang(&lang_name) {
                Ok(_) => println!("setting lang to {lang_name}"),
                Err(e) => eprintln!("{e}"),
            }

            sh.hooks
                .run(sh, ctx, rt, hook_ctx)
                .expect("failed running hook");
        }

        Ok(CmdOutput::success())
    }
}
