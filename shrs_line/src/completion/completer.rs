use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crossterm::style::Stylize;
use relative_path::RelativePath;

use super::{
    drop_path_end, filepaths, find_executables_in_path, path_end, Completer, Completion,
    CompletionCtx,
};

// TODO make this FnMut?
pub type Action = Box<dyn Fn(&CompletionCtx) -> Vec<Completion>>;

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

pub type Filter = Box<dyn Fn(&String) -> bool>;
pub type Format = Box<dyn Fn(String) -> Completion>;

pub struct Rule {
    pub pred: Pred,
    pub action: Action,
    // pub filter: Filter,
    // pub format: Format,
}

impl Rule {
    pub fn new(pred: Pred, action: impl Fn(&CompletionCtx) -> Vec<Completion> + 'static) -> Self {
        Self {
            pred,
            action: Box::new(action),
            // filter:
            // format: Box::new(default_format),
        }
    }

    // TODO this could maybe be rewritten as a builder pattern
}

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

    pub fn complete_helper(&self, ctx: &CompletionCtx) -> Vec<Completion> {
        let rule = self.rules.iter().find(|p| (p.pred).test(ctx));

        match rule {
            Some(rule) => {
                // if rule was matched, run the corresponding action
                // also do prefix search (could make if prefix search is used a config option)
                (rule.action)(ctx)
                    .into_iter()
                    .filter(|s| {
                        s.accept()
                            .starts_with(ctx.cur_word().unwrap_or(&String::new()))
                    })
                    // .map(|s| (rule.format)(s))
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
    fn complete(&self, ctx: &CompletionCtx) -> Vec<Completion> {
        self.complete_helper(ctx)
    }
}

impl Default for DefaultCompleter {
    fn default() -> Self {
        // collection of predefined rules

        let mut comp = DefaultCompleter::new();
        comp.register(Rule::new(
            Pred::new(git_pred).and(flag_pred),
            Box::new(git_flag_action),
        ));
        comp.register(Rule::new(Pred::new(git_pred), Box::new(git_action)));
        comp.register(Rule::new(Pred::new(arg_pred), Box::new(filename_action)));
        comp
    }
}

pub fn cmdname_action(path_str: String) -> impl Fn(&CompletionCtx) -> Vec<Completion> {
    move |ctx: &CompletionCtx| -> Vec<Completion> {
        default_format(find_executables_in_path(&path_str))
    }
}

pub fn filename_action(ctx: &CompletionCtx) -> Vec<Completion> {
    let cur_word = ctx.cur_word().unwrap();
    let drop_end = drop_path_end(cur_word);
    let cur_path = RelativePath::new(&drop_end).to_path(std::env::current_dir().unwrap());

    let output = filepaths(&cur_path).unwrap_or(vec![]);
    output
        .iter()
        .map(|x| {
            let mut filename = x.file_name().unwrap().to_str().unwrap().to_string();

            // append slash if directory name
            let is_dir = x.is_dir();
            if is_dir {
                filename += "/";
            }
            Completion {
                add_space: !is_dir,
                display: Some(filename.to_owned()),
                completion: drop_end.to_owned() + &filename,
            }
        })
        .collect::<Vec<_>>()
}

pub fn git_action(ctx: &CompletionCtx) -> Vec<Completion> {
    default_format(vec!["status".into(), "add".into(), "commit".into()])
}

pub fn git_flag_action(ctx: &CompletionCtx) -> Vec<Completion> {
    default_format(vec!["--version".into(), "--help".into(), "--bare".into()])
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

// TODO temp helper to create a list of completions
pub fn default_format(s: Vec<String>) -> Vec<Completion> {
    s.iter()
        .map(|x| Completion {
            add_space: true,
            display: None,
            completion: x.to_owned(),
        })
        .collect::<Vec<_>>()
}

// pub fn path_format(s: String) -> Completion {
//     Completion { add_space: false, display: Some(path_end(&s)), completion: s }
// }

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
