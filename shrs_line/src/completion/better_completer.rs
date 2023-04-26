use std::collections::HashMap;

use super::{Completer, CompletionCtx};
use crate::completion::filepath_completer;

// TODO make this FnMut?
pub type Action = Box<dyn Fn() -> Vec<String>>;

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
            pred: Box::new(move |ctx: &CompletionCtx| -> bool { (*self.pred)(ctx) || pred(ctx) }),
        }
    }
    pub fn test(&self, ctx: &CompletionCtx) -> bool {
        (self.pred)(ctx)
    }
}

pub struct Rule(pub Pred, pub Action);

/// More advanced completion system that makes use of a collection of [Rule]
pub struct BetterCompleter {
    rules: Vec<Rule>,
}

impl BetterCompleter {
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
                rule.1()
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

impl Completer for BetterCompleter {
    fn complete(&self, ctx: &CompletionCtx) -> Vec<String> {
        self.complete_helper(ctx)
    }
}

impl Default for BetterCompleter {
    fn default() -> Self {
        // collection of predefined rules
        fn cmdname_pred(ctx: &CompletionCtx) -> bool {
            ctx.arg_num() == 0
        }
        fn cmdname_action() -> Vec<String> {
            vec!["vim".into(), "emacs".into(), "nvim".into()]
        }

        fn filename_pred(ctx: &CompletionCtx) -> bool {
            ctx.arg_num() != 0
        }
        fn filename_action() -> Vec<String> {
            filepath_completer()
        }

        fn git_pred(ctx: &CompletionCtx) -> bool {
            cmd_name("git".into())(ctx)
        }
        fn git_action() -> Vec<String> {
            vec!["status".into(), "add".into(), "commit".into()]
        }

        let mut comp = BetterCompleter::new();
        comp.register(Rule(Pred::new(git_pred), Box::new(git_action)));
        comp.register(Rule(
            Pred::new(git_pred).and(is_long_flag),
            Box::new(git_action),
        ));
        comp.register(Rule(Pred::new(cmdname_pred), Box::new(cmdname_action)));
        comp.register(Rule(Pred::new(filename_pred), Box::new(filename_action)));
        comp
    }
}

pub fn cmd_name(cmd_name: String) -> impl Fn(&CompletionCtx) -> bool {
    Box::new(move |ctx: &CompletionCtx| ctx.cmd_name() == Some(&cmd_name))
}

pub fn is_flag(ctx: &CompletionCtx) -> bool {
    is_long_flag(ctx) || is_short_flag(ctx)
}
pub fn is_short_flag(ctx: &CompletionCtx) -> bool {
    todo!()
}
pub fn is_long_flag(ctx: &CompletionCtx) -> bool {
    ctx.cur_word().unwrap_or(&String::new()).starts_with("--")
}

#[cfg(test)]
mod tests {
    use super::{BetterCompleter, Rule};

    #[test]
    fn simple() {
        let mut comp = BetterCompleter::new();
        // comp.register(Rule::new());
    }
}
