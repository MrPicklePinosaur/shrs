use std::error::Error;

use rhai::{Array, Engine, EvalAltResult, Scope};
use shrs::{anyhow::anyhow, prelude::*};

use crate::rhai::RhaiState;

// Run functions defined in rhai script
pub fn command_not_found_hook(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    // TODO this will make defined functions be shadowed by actual commands, not sure if this is
    // desired behaviour

    // TODO also using invalid command exit status is a bit hacky way of adding extra commands to
    // shell

    let Some(state) = sh_ctx.state.get_mut::<RhaiState>() else {
        eprintln!("rhai state not found");
        return Ok(());
    };

    let mut cmd_parts = ctx.command.split(' ');
    let cmd_name = cmd_parts.next().unwrap();

    // search all sourced scripts for function we wish to run
    for ast in state.ast.values() {
        state
            .engine
            .call_fn::<()>(&mut state.scope, ast, cmd_name, ());
    }

    Ok(())
}
pub fn rhai_completions(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &StartupCtx,
) -> anyhow::Result<()> {
    let Some(state) = sh_ctx.state.get_mut::<RhaiState>() else {
        eprintln!("rhai state not found");
        return Ok(());
    };
    //TODO make this a folder
    let compiled = state
        .engine
        .compile_file("/Users/nithin/.config/shrs/completions.rhai".into());
    let ast = match compiled {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Rhai script compile error {}", e);
            return Err(anyhow!("compile error"));
        },
    };
    let name = state
        .engine
        .call_fn::<String>(&mut state.scope, &ast, "name", ())
        .unwrap();
    let completions: Vec<Completion> = default_format(
        state
            .engine
            .call_fn::<Array>(&mut state.scope, &ast, "completions", ())
            .unwrap()
            .iter()
            .map(|d| d.to_string())
            .collect(),
    );
    sh_ctx
        .completer
        .register(Rule::new(Pred::new(cmdname_eq_pred(name)), move |_| {
            completions.clone()
        }));

    Ok(())
}

#[derive(Default)]
pub struct RhaiBuiltin {}

impl RhaiBuiltin {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCmd for RhaiBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let Some(state) = ctx.state.get_mut::<RhaiState>() else {
            eprintln!("Couldnt't get rhai state");
            return Ok(CmdOutput::error());
        };

        for file in args.iter().skip(1) {
            let compiled = state
                .engine
                .compile_file_with_scope(&mut state.scope, file.into());

            let ast = match compiled {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("Rhai script compile error {}", e);
                    return Ok(CmdOutput::error());
                },
            };

            // Always insert in case script file was modified during runtime of shell
            state.ast.insert(file.to_string(), ast.clone());

            state.engine.run_ast_with_scope(&mut state.scope, &ast);
        }

        Ok(CmdOutput::success())
    }
}
