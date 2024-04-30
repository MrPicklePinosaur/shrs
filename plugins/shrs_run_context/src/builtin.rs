use std::{fs::OpenOptions, io::Write};

use clap::Parser;
use shrs::prelude::*;

use crate::RunContextState;

#[derive(Parser)]
struct SaveBuiltinCli {
    context_name: String,
}

pub fn save_builtin(
    mut state: StateMut<RunContextState>,
    rt: State<Runtime>,
    sh: &Shell,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli = SaveBuiltinCli::try_parse_from(args)?;

    state.run_contexts.insert(cli.context_name, rt.clone());

    // save to file if given
    if let Some(context_file) = &state.context_file {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(context_file)
            .unwrap();

        let ron_encoded =
            ron::ser::to_string_pretty(&state.run_contexts, ron::ser::PrettyConfig::default())
                .expect("Error serializing game object");

        file.write_all(ron_encoded.as_bytes()).unwrap();
    }

    Ok(CmdOutput::success())
}

#[derive(Parser)]
struct LoadBuiltinCli {
    context_name: String,
}

pub fn load_builtin(
    mut state: StateMut<RunContextState>,
    mut cmd: StateMut<Commands>,
    mut rt: StateMut<Runtime>,
    sh: &Shell,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli = LoadBuiltinCli::try_parse_from(args)?;

    let mut new_rt: Option<Runtime> = None;
    if let Some(loaded_rt) = state.run_contexts.get(&cli.context_name) {
        new_rt = Some(loaded_rt.clone());
    }

    if let Some(new_rt) = new_rt.take() {
        set_working_dir(sh, &mut rt, &new_rt.working_dir, false).unwrap();
        *rt = new_rt;
    }

    Ok(CmdOutput::success())
}
