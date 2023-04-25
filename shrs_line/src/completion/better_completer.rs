use std::collections::HashMap;

use super::{Completer, CompletionCtx};
use crate::completion::new_filepath_completer;

// TODO make this FnMut?
pub type Pred = dyn Fn(&CompletionCtx) -> bool;
pub type Action = dyn Fn() -> Vec<String>;

pub struct Rule(pub Box<Pred>, pub Box<Action>);

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
        let rule = self.rules.iter().find(|p| (p.0)(ctx));

        match rule {
            Some(rule) => {
                // if rule was matched, run the corresponding action
                rule.1()
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
            new_filepath_completer()
        }

        let mut comp = BetterCompleter::new();
        comp.register(Rule(Box::new(cmdname_pred), Box::new(cmdname_action)));
        comp.register(Rule(Box::new(filename_pred), Box::new(filename_action)));
        comp
    }
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
