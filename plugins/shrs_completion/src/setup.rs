use rhai::{Array, CustomType, Dynamic, Engine, ImmutableString, Scope, Shared, TypeBuilder};
use shrs::prelude::*;
fn default_completion(completion: String) -> Completion {
    Completion {
        add_space: true,
        display: None,
        completion,
        replace_method: ReplaceMethod::Replace,
        comment: None,
    }
}
fn default_completion_with_comment(completion: String, comment: String) -> Completion {
    Completion {
        add_space: true,
        display: None,
        completion,
        replace_method: ReplaceMethod::Replace,
        comment: Some(comment),
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
fn arg_num(ctx: &mut CompletionCtx) -> i64 {
    ctx.arg_num() as i64
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
        .map(|s| s.clone().into_typed_array::<String>().unwrap())
        .map(|a| (a.first().unwrap().clone(), a.last().unwrap().clone()))
        .collect();
    let g = default_format_with_comment(c);
    g.iter().map(|f| Dynamic::from(f.clone())).collect()
}
fn filename_completions(ctx: CompletionCtx) -> Array {
    filename_action(&ctx)
        .iter()
        .map(|f| Dynamic::from(f.clone()))
        .collect()
}

pub fn setup_engine(engine: &mut Engine) {
    engine
        .register_type::<Completion>()
        .register_get("completion", |c: &mut Completion| c.completion.clone());
    engine.register_type::<ReplaceMethod>();
    engine
        .register_type::<CompletionCtx>()
        .register_get("line", get_line)
        .register_get("cmd_name", cmd_name)
        .register_get("cur_word", cur_word)
        .register_get("arg_num", arg_num);

    //create completion
    engine.register_fn("Completion", new_completion);
    engine.register_fn("Completion", default_completion);
    engine.register_fn("Completion", default_completion_with_comment);

    //ReplaceMethod enum
    engine.register_fn("Append", append_method);
    engine.register_fn("Replace", replace_method);

    //Vec<Completion> utilities
    engine.register_fn("default_format", df);
    engine.register_fn("default_format_with_comment", dfc);
    engine.register_fn("filename_completions", filename_completions);
    //have to do this because Rhai calls with CompletionCtx instead of &CompletionCtx
    //need way to combine short and long flag
    //Pred Utilities
    engine.register_fn("is_short_flag", |c| short_flag_pred(&c));
    engine.register_fn("is_long_flag", |c| long_flag_pred(&c));
    engine.register_fn("is_cmdname", |c| cmdname_pred(&c));
    engine.register_fn("is_path", |c| path_pred(&c));
    engine.register_fn("is_arg", |c| arg_pred(&c));
}
