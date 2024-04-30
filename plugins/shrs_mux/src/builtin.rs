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

pub fn mux_builtin(
    mut mux_state: StateMut<MuxState>,
    sh: &Shell,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;

    if cli.list {
        for (lang_name, _) in mux_state.iter() {
            println!("{lang_name}")
        }
    }

    if let Some(lang_name) = cli.lang {
        let old_lang = mux_state.current_lang();
        let hook_ctx = ChangeLangCtx {
            old_lang: old_lang.name(),
            new_lang: lang_name.clone().into(),
        };

        if let Err(e) = mux_state.set_current_lang(&lang_name) {
            eprintln!("{e}");
            return Ok(CmdOutput::error());
        }

        println!("setting lang to {lang_name}");

        // HACK, prime the language so it can run any init that it needs (this is to support
        // lazy loading languages)
        sh.cmd.eval("");
        sh.cmd.run_hook(hook_ctx);
    }
    Ok(CmdOutput::success())
}
