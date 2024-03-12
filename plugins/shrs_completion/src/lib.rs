//! More completions for shrs
//!
//!

pub mod completions;
mod helpers;

use ::anyhow::anyhow;
use rhai::{Array, CustomType, Dynamic, Engine, Scope, TypeBuilder};
use shrs::prelude::*;

pub struct CompletionsPlugin;

#[derive(Clone, Debug, CustomType)]
pub struct RhaiCompletion {
    long: String,
    short: String,
    description: String,
}

impl Plugin for CompletionsPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.hooks.insert(rhai_completions);
        Ok(())
    }
}
pub fn rhai_completions(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &StartupCtx,
) -> anyhow::Result<()> {
    let mut engine = Engine::new();
    let mut scope = Scope::new();
    engine.build_type::<RhaiCompletion>();
    engine.register_fn(
        "create_completion",
        |long: &str, short: &str, description: &str| -> RhaiCompletion {
            RhaiCompletion {
                long: long.to_string(),
                short: short.to_string(),
                description: description.to_string(),
            }
        },
    );

    //TODO make this a folder
    let compiled = engine.compile_file_with_scope(
        &mut scope,
        "/Users/nithin/.config/shrs/completions.rhai".into(),
    );
    let ast = match compiled {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Rhai script compile error {}", e);
            return Err(anyhow!("compile error"));
        },
    };
    let completions: Vec<RhaiCompletion> = engine
        .call_fn::<Array>(&mut scope, &ast, "completions", ())
        .unwrap()
        .iter()
        .map(|x| x.clone().cast::<RhaiCompletion>())
        .collect();
    let name: String = engine
        .call_fn::<String>(&mut scope, &ast, "name", ())
        .unwrap();
    dbg!(completions);
    dbg!(name);

    // sh_ctx
    //     .completer
    //     .register(Rule::new(Pred::new(cmdname_eq_pred(name)), move |_| {
    //         completions.clone()
    //     }));

    Ok(())
}
