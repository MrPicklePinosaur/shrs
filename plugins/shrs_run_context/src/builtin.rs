use std::{fs::OpenOptions, io::Write};

use clap::Parser;
use shrs::prelude::*;

use crate::RunContextState;

pub struct SaveBuiltin;

#[derive(Parser)]
struct SaveBuiltinCli {
    context_name: String,
}

impl BuiltinCmd for SaveBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let cli = SaveBuiltinCli::try_parse_from(args)?;

        if let Some(state) = ctx.state.get_mut::<RunContextState>() {
            state.run_contexts.insert(cli.context_name, rt.clone());

            // save to file if given
            if let Some(context_file) = &state.context_file {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(context_file)
                    .unwrap();

                let ron_encoded = ron::ser::to_string_pretty(
                    &state.run_contexts,
                    ron::ser::PrettyConfig::default(),
                )
                .expect("Error serializing game object");

                file.write_all(ron_encoded.as_bytes()).unwrap();
            }
        }

        Ok(CmdOutput::success())
    }
}

pub struct LoadBuiltin;

#[derive(Parser)]
struct LoadBuiltinCli {
    context_name: String,
}

impl BuiltinCmd for LoadBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        use std::mem;

        let cli = LoadBuiltinCli::parse_from(["load".to_string()].iter().chain(args.iter()));

        let mut new_rt: Option<Runtime> = None;
        if let Some(state) = ctx.state.get_mut::<RunContextState>() {
            if let Some(loaded_rt) = state.run_contexts.get(&cli.context_name) {
                new_rt = Some(loaded_rt.clone());
            }
        }

        if let Some(new_rt) = new_rt.take() {
            set_working_dir(sh, ctx, rt, &new_rt.working_dir, false).unwrap();
            let _ = mem::replace(rt, new_rt);
        }

        Ok(CmdOutput::success())
    }
}
