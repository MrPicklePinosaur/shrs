use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crossterm::style::Stylize;
use relative_path::RelativePath;

use super::{drop_path_end, filepaths, find_executables_in_path, Completer, CompletionCtx};

// TODO make this FnMut?
pub type Action = Box<dyn Fn(&CompletionCtx) -> Vec<String>>;

pub struct Pred {
    pred: Box<dyn Fn(&CompletionCtx) -> bool>,
}

impl Pred {
    pub fn new(pred: impl Fn(&CompletionCtx) -> bool + 'static) -> Self {
        Self {
            pred: Box::new(pred),
        }
    }
    pub fn and(self, pred: impl Fn(&CompletionCtx) -> bool + 'static) -> Self {
        Self {
            pred: Box::new(move |ctx: &CompletionCtx| -> bool { (*self.pred)(ctx) && pred(ctx) }),
        }
    }
    pub fn test(&self, ctx: &CompletionCtx) -> bool {
        (self.pred)(ctx)
    }
}

pub struct Rule(pub Pred, pub Action);

pub struct DefaultCompleter {
    rules: Vec<Rule>,
}

impl DefaultCompleter {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    /// Register a new rule to use
    pub fn register(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn complete_helper(&self, ctx: &CompletionCtx) -> Vec<String> {
        let rule = self.rules.iter().find(|p| (p.0).test(ctx));

        match rule {
            Some(rule) => {
                // if rule was matched, run the corresponding action
                // also do prefix search (could make if prefix search is used a config option)
                rule.1(ctx)
                    .into_iter()
                    .filter(|s| s.starts_with(ctx.cur_word().unwrap_or(&String::new())))
                    .collect::<Vec<_>>()
            },
            None => {
                // TODO display some notif that we cannot complete
                vec![]
            },
        }
    }
}

impl Completer for DefaultCompleter {
    fn complete(&self, ctx: &CompletionCtx) -> Vec<String> {
        self.complete_helper(ctx)
    }
}

impl Default for DefaultCompleter {
    fn default() -> Self {
        // collection of predefined rules

        let mut comp = DefaultCompleter::new();
        comp.register(Rule(
            Pred::new(git_pred).and(flag_pred),
            Box::new(git_flag_action),
        ));
        comp.register(Rule(Pred::new(git_pred), Box::new(git_action)));
        comp.register(Rule(Pred::new(arg_pred), Box::new(filename_action)));
        comp
    }
}

pub fn cmdname_action(path_str: String) -> impl Fn(&CompletionCtx) -> Vec<String> {
    move |ctx: &CompletionCtx| -> Vec<String> { find_executables_in_path(&path_str) }
}

pub fn filename_action(ctx: &CompletionCtx) -> Vec<String> {
    let cur_word = ctx.cur_word().unwrap();
    let cur_path =
        RelativePath::new(&drop_path_end(cur_word)).to_path(std::env::current_dir().unwrap());

    filepaths(&cur_path).unwrap_or(vec!["invlad".into()])
}

pub fn git_action(ctx: &CompletionCtx) -> Vec<String> {
    vec!["status".into(), "add".into(), "commit".into()]
}

pub fn git_flag_action(ctx: &CompletionCtx) -> Vec<String> {
    vec!["--version".into(), "--help".into(), "--bare".into()]
}

/// Check if we are completing the command name
pub fn cmdname_pred(ctx: &CompletionCtx) -> bool {
    ctx.arg_num() == 0
}
pub fn git_pred(ctx: &CompletionCtx) -> bool {
    cmdname_eq_pred("git".into())(ctx)
}

/// Check if we are attempting to complete an argument
pub fn arg_pred(ctx: &CompletionCtx) -> bool {
    ctx.arg_num() != 0
}

/// Check if name of current command equals a given command name
pub fn cmdname_eq_pred(cmd_name: String) -> impl Fn(&CompletionCtx) -> bool {
    move |ctx: &CompletionCtx| ctx.cmd_name() == Some(&cmd_name)
}

/// Check if we are completing a flag
pub fn flag_pred(ctx: &CompletionCtx) -> bool {
    long_flag_pred(ctx) || short_flag_pred(ctx)
}
pub fn short_flag_pred(ctx: &CompletionCtx) -> bool {
    ctx.cur_word().unwrap_or(&String::new()).starts_with("-") && !long_flag_pred(ctx)
}
pub fn long_flag_pred(ctx: &CompletionCtx) -> bool {
    ctx.cur_word().unwrap_or(&String::new()).starts_with("--")
}

/// Check if we are completing a (real) path
pub fn path_pred(ctx: &CompletionCtx) -> bool {
    // strip part after slash
    let cur_word = ctx.cur_word().unwrap();
    let cur_path =
        RelativePath::new(&drop_path_end(cur_word)).to_path(std::env::current_dir().unwrap());

    cur_path.is_dir()
}

#[cfg(test)]
mod tests {
    use super::{flag_pred, path_pred, DefaultCompleter, Rule};
    use crate::completion::CompletionCtx;

    #[test]
    fn simple() {
        let mut comp = DefaultCompleter::new();
        // comp.register(Rule::new());
    }

    #[test]
    fn test_is_flag() {
        let ctx = CompletionCtx::new(vec!["git".into(), "-".into()]);
        assert!(flag_pred(&ctx));
        let ctx = CompletionCtx::new(vec![]);
        assert!(!flag_pred(&ctx));
    }
}
