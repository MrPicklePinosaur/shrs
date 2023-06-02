//! Demonstration of conditional aliases
//!
//! If the user is in the home directory, the alias will evaluate to the command `true`, otherwise
//! the alias will evaluate to the command `false`

use shrs::prelude::*;

// TODO need better example, this one is pretty hacky
fn in_home_directory() -> bool {
    top_pwd() == "~"
}

fn main() {
    let mut alias = Alias::new();
    alias.set(
        "inhome",
        AliasInfo::with_rule("true", |ctx| in_home_directory()),
    );
    alias.set(
        "inhome",
        AliasInfo::with_rule("false", |ctx| !in_home_directory()),
    );

    let myshell = ShellConfigBuilder::default()
        .with_alias(alias)
        .build()
        .unwrap();

    myshell.run();
}
