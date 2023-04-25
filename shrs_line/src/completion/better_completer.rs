use std::collections::HashMap;

use super::{Completer, CompletionCtx};

// TODO make this FnMut?
pub type Pred = dyn Fn(&CompletionCtx) -> bool;
pub type Action = dyn Fn() -> Vec<String>;

pub struct Rule(pub Box<Pred>, pub Box<Action>);

impl Rule {
    pub fn new(pred: &'static Pred, action: &'static Action) -> Self {
        Rule(Box::new(pred), Box::new(action))
    }
}

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

#[cfg(test)]
mod tests {
    use super::{BetterCompleter, Rule};

    #[test]
    fn simple() {
        let mut comp = BetterCompleter::new();
        // comp.register(Rule::new());
    }
}
