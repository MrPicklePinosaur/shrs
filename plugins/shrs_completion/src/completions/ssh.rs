use shrs::prelude::{cmdname_eq_pred, Completion, CompletionCtx, Pred, ReplaceMethod, Rule};

use crate::helpers::known_hosts;

// TODO this only works with the default completer, perhaps make register part of the completer
// trait?
pub fn ssh_rule() -> Rule {
    Rule::new(Pred::new(ssh_pred), Box::new(known_hosts_action))
}

fn ssh_pred(ctx: &CompletionCtx) -> bool {
    cmdname_eq_pred("ssh".into())(ctx)
}

fn known_hosts_action(_ctx: &CompletionCtx) -> Vec<Completion> {
    // TODO currently hardcoded config path
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".ssh/config");
    let hosts = known_hosts(&config_path).unwrap();

    let completions = hosts
        .iter()
        .map(|host| {
            host.pattern
                .iter()
                .map(|pat| Completion {
                    add_space: true,
                    display: Some(pat.pattern.clone()),
                    completion: pat.pattern.clone(),
                    replace_method: ReplaceMethod::Append,
                    comment: None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    completions
}
