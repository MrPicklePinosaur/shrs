//! More completions for shrs
//!
//!

pub mod completions;
mod helpers;

use ::anyhow::anyhow;
use rhai::{Array, CustomType, Dynamic, Engine, ImmutableString, Scope, TypeBuilder};
use shrs::prelude::*;

pub struct CompletionsPlugin;

impl Plugin for CompletionsPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.hooks.insert(rhai_completions);
        Ok(())
    }
}
fn new_completion(
    add_space: bool,
    display: String,
    completion: String,
    replace_method: ReplaceMethod,
    comment: String,
) -> Completion {
    Completion {
        add_space,
        display: Some(display),
        completion,
        replace_method,
        comment: Some(comment),
    }
}
fn append_method() -> ReplaceMethod {
    ReplaceMethod::Append
}
fn replace_method() -> ReplaceMethod {
    ReplaceMethod::Replace
}
fn get_line(ctx: &mut CompletionCtx) -> Vec<String> {
    ctx.line.clone()
}
fn cmd_name(ctx: &mut CompletionCtx) -> Dynamic {
    if let Some(n) = ctx.cmd_name() {
        return n.into();
    }
    Dynamic::UNIT
}
fn cur_word(ctx: &mut CompletionCtx) -> Dynamic {
    if let Some(w) = ctx.cur_word() {
        return w.into();
    }
    Dynamic::UNIT
}
fn arg_num(ctx: &mut CompletionCtx) -> usize {
    ctx.arg_num()
}
fn df(arr: Array) -> Array {
    let c: Vec<String> = arr
        .iter()
        .map(|s| s.clone().into_string().unwrap())
        .collect();
    let g = default_format(c);
    g.iter().map(|f| Dynamic::from(f.clone())).collect()
}
fn dfc(arr: Array) -> Array {
    let c: Vec<(String, String)> = arr
        .iter()
        .map(|s| s.clone().cast::<(String, String)>())
        .collect();
    let g = default_format_with_comment(c);
    g.iter().map(|f| Dynamic::from(f.clone())).collect()
}

pub fn setup_engine(engine: &mut Engine) {
    engine.register_type::<Completion>();
    engine.register_type::<ReplaceMethod>();
    engine
        .register_type::<CompletionCtx>()
        .register_get("line", get_line)
        .register_get("cmd_name", cmd_name)
        .register_get("cur_word", cur_word)
        .register_get("arg_num", arg_num);

    engine.register_fn("Completion", new_completion);
    engine.register_fn("Append", append_method);
    engine.register_fn("Replace", replace_method);
    engine.register_fn("default_format", df);
    engine.register_fn("default_format_with_comment", dfc);
}

pub fn rhai_completions(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &StartupCtx,
) -> anyhow::Result<()> {
    sh_ctx.completer.register(Rule::new(
        Pred::new(move |c| {
            let mut engine = Engine::new();
            setup_engine(&mut engine);

            let mut scope = Scope::new();

            //TODO make this a folder
            let compiled = engine.compile_file_with_scope(
                &mut scope,
                "/Users/nithin/.config/shrs/completions.rhai".into(),
            );
            let ast = match compiled {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("Rhai script compile error {}", e);
                    return false;
                },
            };

            let predicate: bool = engine
                .call_fn::<bool>(&mut scope, &ast, "predicate", (c.clone(),))
                .unwrap();
            predicate
        }),
        |c| -> Vec<Completion> {
            let mut engine = Engine::new();
            setup_engine(&mut engine);
            let mut scope = Scope::new();

            //TODO make this a folder
            let compiled = engine.compile_file_with_scope(
                &mut scope,
                "/Users/nithin/.config/shrs/completions.rhai".into(),
            );
            let ast = match compiled {
                Ok(ast) => ast,
                Err(e) => {
                    eprintln!("Rhai script compile error {}", e);
                    return vec![];
                },
            };

            let completions: Vec<Completion> = engine
                .call_fn::<Array>(&mut scope, &ast, "completions", (c.clone(),))
                .unwrap()
                .iter()
                .map(|x| x.clone().cast::<Completion>())
                .collect();
            completions
        },
    ));

    Ok(())
}
