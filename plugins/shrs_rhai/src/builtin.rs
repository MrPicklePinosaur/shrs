use log::error;
use shrs::prelude::*;

use crate::rhai::{RhaiAST, RhaiEngine, RhaiScope};

// Run functions defined in rhai script
pub fn command_not_found_hook(
    rhai_ast: State<RhaiAST>,
    mut scope: StateMut<RhaiScope>,
    engine: State<RhaiEngine>,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    // TODO this will make defined functions be shadowed by actual commands, not sure if this is
    // desired behaviour

    // TODO also using invalid command exit status is a bit hacky way of adding extra commands to
    // shell

    let mut cmd_parts = ctx.command.split(' ');
    let cmd_name = cmd_parts.next().unwrap();

    // search all sourced scripts for function we wish to run
    for ast in rhai_ast.values() {
        let _r = engine.call_fn::<()>(&mut scope, ast, cmd_name, ());
    }

    Ok(())
}

pub fn rhai_builtin(
    mut scope: StateMut<RhaiScope>,
    engine: State<RhaiEngine>,
    mut rhai_ast: StateMut<RhaiAST>,
    sh: &Shell,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    for file in args.iter().skip(1) {
        let compiled = engine.compile_file_with_scope(&mut scope, file.into());

        let ast = match compiled {
            Ok(ast) => ast,
            Err(e) => {
                error!("Rhai script compile error {}", e);
                return Ok(CmdOutput::error());
            },
        };

        // Always insert in case script file was modified during runtime of shell
        rhai_ast.insert(file.to_string(), ast.clone());

        if let Err(e) = engine.run_ast_with_scope(&mut scope, &ast) {
            eprintln!("rhai eval error: {e}")
        }
    }

    Ok(CmdOutput::success())
}
