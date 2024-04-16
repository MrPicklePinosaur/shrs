use rhai::{Array, Dynamic, Engine};
use shrs::prelude::*;

/// Constructor utilites for Completions
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
    display: Dynamic,
    completion: String,
    replace_method: ReplaceMethod,
    comment: Dynamic,
) -> Completion {
    Completion {
        add_space,
        display: if display.is_unit() {
            None
        } else {
            Some(display.into_string().unwrap())
        },
        completion,
        replace_method,
        comment: if comment.is_unit() {
            None
        } else {
            Some(comment.into_string().unwrap())
        },
    }
}

/// Enum constructors
fn append_method() -> ReplaceMethod {
    ReplaceMethod::Append
}

fn replace_method() -> ReplaceMethod {
    ReplaceMethod::Replace
}

///CompletionCtx Getters
fn get_line(ctx: &mut CompletionCtx) -> Array {
    ctx.line.iter().map(|f| Dynamic::from(f.clone())).collect()
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

fn with_format_default(arr: Array) -> Array {
    with_format(arr, true, ReplaceMethod::Replace)
}
fn with_format(arr: Array, add_space: bool, replace_method: ReplaceMethod) -> Array {
    arr.iter()
        .map(|s| {
            if s.is_string() {
                Completion {
                    add_space,
                    display: None,
                    completion: s.clone().cast(),
                    replace_method,
                    comment: None,
                }
            } else if s.is::<Completion>() {
                s.clone_cast::<Completion>()
            } else if s.is_array() {
                let c = s.clone().into_typed_array::<String>().unwrap();
                Completion {
                    add_space,
                    display: None,
                    completion: c.first().unwrap().into(),
                    replace_method,
                    comment: Some(c.last().unwrap().into()),
                }
            } else {
                panic!("Incorrect type {}", s.type_name());
            }
        })
        .map(Dynamic::from)
        .collect()
}

/// Returns all file names in cwd
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

    // create completion
    engine.register_fn("Completion", new_completion);
    engine.register_fn("Completion", default_completion);
    engine.register_fn("Completion", default_completion_with_comment);

    // ReplaceMethod enum
    engine.register_fn("Append", append_method);
    engine.register_fn("Replace", replace_method);

    // Vec<Completion> utilities
    engine.register_fn("with_format", with_format);
    engine.register_fn("with_format", with_format_default);

    engine.register_fn("filename_completions", filename_completions);
    //  Filters flags from Array of strings
    engine.register_fn("filter_flags", |line: Array| -> Array {
        line.iter()
            .map(|s| s.clone().into_string().unwrap())
            .filter(|v| !v.starts_with("-"))
            .map(Dynamic::from)
            .collect()
    });
    // have to do this because Rhai calls with CompletionCtx instead of &CompletionCtx
    // need way to combine short and long flag
    // Pred Utilities
    engine.register_fn("is_short_flag", |c| short_flag_pred(&c));
    engine.register_fn("is_long_flag", |c| long_flag_pred(&c));
    engine.register_fn("is_cmdname", |c| cmdname_pred(&c));
    engine.register_fn("is_path", |c| path_pred(&c));
    engine.register_fn("is_arg", |c| arg_pred(&c));
}
