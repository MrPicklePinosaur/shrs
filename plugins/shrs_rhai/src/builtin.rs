use shrs::prelude::*;

use crate::rhai::RhaiState;

// Run functions defined in rhai script
pub fn command_not_found_hook(
    _sh: &Shell,
    sh_ctx: &mut States,
    _sh_rt: &mut Runtime,
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
        let _r = state
            .engine
            .call_fn::<()>(&mut state.scope, ast, cmd_name, ());
    }

    Ok(())
}

#[derive(Default)]
pub struct RhaiBuiltin {}

impl RhaiBuiltin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Builtin for RhaiBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        ctx: &mut States,
        _rt: &mut Runtime,
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

            state
                .engine
                .run_ast_with_scope(&mut state.scope, &ast)
                .expect("unable to run ast");
        }

        Ok(CmdOutput::success())
    }
}
