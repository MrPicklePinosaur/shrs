//! Implementation of default rule based completer

use std::path::{Path, PathBuf};

use super::{
    data::*, drop_path_end, filepaths, find_executables_in_path, Completer, Completion,
    CompletionCtx, ReplaceMethod,
};
use crate::prelude::Builtins;

// TODO make this FnMut?
/// Actions return a list of possible completions
pub type Action = Box<dyn Fn(&CompletionCtx) -> Vec<Completion>>;

/// Predicates are functions that take in some context and return a boolean
pub struct Pred {
    pred: Box<dyn Fn(&CompletionCtx) -> bool>,
}

impl Pred {
    /// Construct a new predicate from a function
    pub fn new(pred: impl Fn(&CompletionCtx) -> bool + 'static) -> Self {
        Self {
            pred: Box::new(pred),
        }
    }
    /// Chain another predicate
    ///
    /// The predicates short circuit
    pub fn and(self, pred: impl Fn(&CompletionCtx) -> bool + 'static) -> Self {
        Self {
            pred: Box::new(move |ctx: &CompletionCtx| -> bool { (*self.pred)(ctx) && pred(ctx) }),
        }
    }
    /// Check if predicate is satisfied
    pub fn test(&self, ctx: &CompletionCtx) -> bool {
        (self.pred)(ctx)
    }
}

pub type Filter = Box<dyn Fn(&String) -> bool>;
pub type Format = Box<dyn Fn(String) -> Completion>;

/// Rules for [DefaultCompleter]
pub struct Rule {
    /// Predicate to check
    pub pred: Pred,
    /// Action to execute if predicate is satisfied
    pub completions: Action,
    // pub filter: Filter,
    // pub format: Format,
}

impl Rule {
    pub fn new(pred: Pred, action: impl Fn(&CompletionCtx) -> Vec<Completion> + 'static) -> Self {
        Self {
            pred,
            completions: Box::new(action),
            // filter:
            // format: Box::new(default_format),
        }
    }

    // TODO this could maybe be rewritten as a builder pattern
}

/// Default rule-based completion system
pub struct DefaultCompleter {
    rules: Vec<Rule>,
}

impl DefaultCompleter {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    fn complete_helper(&self, ctx: &CompletionCtx) -> Vec<Completion> {
        let rules: Vec<&Rule> = self.rules.iter().filter(|p| (p.pred).test(ctx)).collect();

        let mut output = vec![];
        //if no rules were matched, default to files in the current folder
        if rules.is_empty() {
            return filename_action(ctx)
                .into_iter()
                .filter(|s| {
                    s.accept()
                        .starts_with(ctx.cur_word().unwrap_or(&String::new()))
                })
                // .map(|s| (rule.format)(s))
                .collect::<Vec<_>>();
        }

        for rule in rules {
            // if rule was matched, run the corresponding action
            // also do prefix search (could make if prefix search is used a config option)
            let mut comps = (rule.completions)(ctx)
                .into_iter()
                .filter(|s| {
                    s.accept()
                        .starts_with(ctx.cur_word().unwrap_or(&String::new()))
                })
                // .map(|s| (rule.format)(s))
                .collect::<Vec<_>>();

            output.append(&mut comps);
        }
        output
    }
}

impl Completer for DefaultCompleter {
    fn complete(&self, ctx: &CompletionCtx) -> Vec<Completion> {
        self.complete_helper(ctx)
    }
    /// Register a new rule to use
    fn register(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
}

impl Default for DefaultCompleter {
    /// Register default rules
    fn default() -> Self {
        // collection of predefined rules

        let mut comp = DefaultCompleter::new();
        comp.register(Rule::new(
            Pred::new(ls_pred).and(short_flag_pred),
            Box::new(ls_short_flag_action),
        ));
        comp.register(Rule::new(
            Pred::new(ls_pred).and(long_flag_pred),
            Box::new(ls_long_flag_action),
        ));
        comp
    }
}

/// Return all the executables in PATH
pub fn cmdname_action(path_str: String) -> impl Fn(&CompletionCtx) -> Vec<Completion> {
    move |_ctx: &CompletionCtx| -> Vec<Completion> {
        default_format(find_executables_in_path(&path_str))
    }
}

/// Return all the builtin command names
pub fn builtin_cmdname_action(builtin: &Builtins) -> impl Fn(&CompletionCtx) -> Vec<Completion> {
    let builtin_names = builtin
        .iter()
        .map(|(name, _)| name.to_owned().to_string())
        .collect::<Vec<_>>();
    move |_ctx: &CompletionCtx| -> Vec<Completion> { default_format(builtin_names.clone()) }
}

/// Look in current directory for potential filenames to complete
pub fn filename_action(ctx: &CompletionCtx) -> Vec<Completion> {
    let cur_word = ctx.cur_word().unwrap();
    let drop_end = drop_path_end(cur_word);
    let cur_path = to_absolute(&drop_end, &dirs::home_dir().unwrap());

    let output = filepaths(&cur_path).unwrap_or_default();
    output
        .iter()
        .map(|x| {
            let filename = x.file_name().unwrap().to_str().unwrap().to_string();

            let mut filename = sanitize_file_name(filename);

            // escape special characters in filename

            // append slash if directory name
            let is_dir = x.is_dir();
            if is_dir {
                filename += "/";
            }
            Completion {
                add_space: !is_dir,
                display: Some(filename.to_owned()),
                completion: drop_end.to_owned() + &filename,
                replace_method: ReplaceMethod::Replace,
                comment: None,
            }
        })
        .collect::<Vec<_>>()
}

/// Takes in an arbitrary path that user enters and convert it into an absolute path
fn to_absolute(path_str: &str, home_dir: &Path) -> PathBuf {
    let path_buf = PathBuf::from(path_str);

    let absolute = if path_buf.has_root() {
        path_buf
    } else {
        // handle home directory tilde
        // TODO ~username/ not yet handled
        if let Ok(stripped) = path_buf.strip_prefix("~/") {
            home_dir.join(stripped)
        } else {
            std::env::current_dir().unwrap().join(path_buf)
        }
    };

    absolute
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
/// Check if we are completing a short flag
pub fn short_flag_pred(ctx: &CompletionCtx) -> bool {
    ctx.cur_word().unwrap_or(&String::new()).starts_with('-') && !long_flag_pred(ctx)
}
/// Check if we are completing a long flag
pub fn long_flag_pred(ctx: &CompletionCtx) -> bool {
    ctx.cur_word().unwrap_or(&String::new()).starts_with("--")
}

/// Check if we are completing a (real) path
pub fn path_pred(ctx: &CompletionCtx) -> bool {
    // strip part after slash
    let cur_word = ctx.cur_word().unwrap();
    // TODO should technically be using HOME env variable?
    let cur_path = to_absolute(&drop_path_end(cur_word), &dirs::home_dir().unwrap());

    cur_path.is_dir()
}

// TODO temp helper to create a list of completions
/// Construct a [Completion] with default options
pub fn default_format(s: Vec<String>) -> Vec<Completion> {
    s.iter()
        .map(|x| Completion {
            add_space: true,
            display: None,
            completion: x.to_owned(),
            replace_method: ReplaceMethod::Replace,
            comment: None,
        })
        .collect::<Vec<_>>()
}

pub fn default_format_with_comment(s: Vec<(String, String)>) -> Vec<Completion> {
    s.iter()
        .map(|x| Completion {
            add_space: true,
            display: None,
            completion: x.0.to_string(),
            replace_method: ReplaceMethod::Replace,
            comment: Some(x.1.to_string()),
        })
        .collect::<Vec<_>>()
}

// pub fn path_format(s: String) -> Completion {
//     Completion { add_space: false, display: Some(path_end(&s)), completion: s }
// }

fn sanitize_file_name(filename: String) -> String {
    // lazy_static! {
    //     static ref ESCAPE_SPACE
    // }

    filename.replace(' ', "\\ ")
}

#[cfg(test)]
mod tests {
    use super::{flag_pred, DefaultCompleter};
    use crate::completion::CompletionCtx;

    #[test]
    fn simple() {
        let _comp = DefaultCompleter::new();
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
