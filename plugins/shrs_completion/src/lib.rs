//! More completions for shrs
//!
//!

pub mod completions;
mod helpers;
use std::{fs, path::PathBuf, rc::Rc};

use ::anyhow::anyhow;
use rhai::{Array, CustomType, Dynamic, Engine, ImmutableString, Scope, Shared, TypeBuilder};
use shrs::prelude::*;
pub struct CompletionsState {
    pub engine: Engine,
}

impl CompletionsState {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        setup_engine(&mut engine);

        CompletionsState { engine }
    }
}

pub struct CompletionsPlugin;

impl Plugin for CompletionsPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.state.insert(CompletionsState::new());
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
    //have to do this because Rhai calls with CompletionCtx instead of &CompletionCtx
    engine.register_fn("is_short_flag", |c| short_flag_pred(&c));
    engine.register_fn("is_long_flag", |c| long_flag_pred(&c));
    engine.register_fn("is_cmdname", |c| cmdname_pred(&c));
    engine.register_fn("is_path", |c| path_pred(&c));
    engine.register_fn("is_arg", |c| arg_pred(&c));
}

pub fn rhai_completions(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &StartupCtx,
) -> anyhow::Result<()> {
    let Some(state) = sh_ctx.state.get::<CompletionsState>() else {
        eprintln!("rhai state not found");
        return Ok(());
    };

    let mut e = Engine::new();
    setup_engine(&mut e);
    let engine = Rc::new(e);
    let folder: PathBuf = "/Users/nithin/.config/shrs/completions".into();
    for p in fs::read_dir(folder).unwrap() {
        let path = p.unwrap().path();
        let compiled = engine.compile_file(path);
        let ast = Rc::new(match compiled {
            Ok(ast) => ast,
            Err(e) => {
                eprintln!("Rhai script compile error {}", e);
                return Err(anyhow!("Can compile"));
            },
        });
        let ast1 = ast.clone();

        let e1 = engine.clone();
        let e2 = engine.clone();

        sh_ctx.completer.register(Rule::new(
            Pred::new(move |c| {
                let mut scope = Scope::new();

                let predicate: bool = e1
                    .call_fn::<bool>(&mut scope, &ast, "predicate", (c.clone(),))
                    .unwrap();
                predicate
            }),
            move |c| -> Vec<Completion> {
                let mut scope = Scope::new();

                let completions: Vec<Completion> = e2
                    .call_fn::<Array>(&mut scope, &ast1, "completions", (c.clone(),))
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().cast::<Completion>())
                    .collect();
                completions
            },
        ));
    }

    Ok(())
}
