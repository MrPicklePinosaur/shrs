use clap::{Parser, Subcommand};
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
        args: &Vec<String>,
    ) -> anyhow::Result<BuiltinStatus> {
        let cli = SaveBuiltinCli::parse_from(vec!["save".to_string()].iter().chain(args.iter()));

        if let Some(state) = ctx.state.get_mut::<RunContextState>() {
            state.run_contexts.insert(cli.context_name, rt.clone());
        }

        Ok(BuiltinStatus::success())
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
        _sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<BuiltinStatus> {
        use std::mem;

        let cli = LoadBuiltinCli::parse_from(vec!["load".to_string()].iter().chain(args.iter()));

        if let Some(state) = ctx.state.get_mut::<RunContextState>() {
            if let Some(loaded_rt) = state.run_contexts.get(&cli.context_name) {
                let _ = mem::replace(rt, loaded_rt.clone());
            }
        }

        Ok(BuiltinStatus::success())
    }
}
