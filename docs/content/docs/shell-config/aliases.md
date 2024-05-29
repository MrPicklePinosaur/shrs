+++
title = "Aliases"
description = ""
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ''
toc = true
top = false
+++

Aliases can be specified as a key value pair of the alias name and the actual command it expands to. Keep in mind that aliases are not evaluated or syntax checked at time of definition, only during substitution. This means that it is possible to define aliases that are invalid commands. The easiest way to set aliases is by using `Alias::from_iter()` to set multiple at once.
```rust
let alias = Alias::from_iter([
    ("l", "ls"),
    ("c", "cd"),
    ("g", "git"),
    ("v", "vim"),
    ("la", "ls -a"),
]);

myshell.with_alias(alias);
```

It is also possible to set one alias at a time using `Alias::set()`, which let's you employ more complex control flow when setting aliases. This is the equivalent of the above:
```rust
let mut alias = Alias::new();

alias.set("l", AliasInfo::always("ls"));
alias.set("c", AliasInfo::always("cd"));
alias.set("g", AliasInfo::always("git"));
alias.set("v", AliasInfo::always("vim"));
alias.set("la", AliasInfo::always("ls -a"));
```

The `AliasInfo` lets you *conditionally* register aliases, `AliasInfo::always` will produce an alias that is always active.

## Conditional Aliases

There is also currently an experimental feature to have aliases by activated conditionally based on a predicate. This allows you to enable/disable groups of aliases at runtime, for example only enable git aliases when in a git repo. It is not yet supported to add conditional aliases using `Alias::from_iter()`, so you must use the `Alias::set()` syntax. The below shows how you can make your `ls` rainbow only on Fridays:
```rust
use chrono::{Datelike, Local, Weekday};

let mut alias = Alias::new();

let ls_alias = AliasInfo::with_rule("ls | lolcat", |ctx: &AliasRuleCtx| -> bool {
    let weekday = Local::now().weekday();
    weekday == Weekday::Fri
});
alias.set("ls", ls_alias);
```
`AliasRuleCtx` gives you access to shell state when deciding if the alias should be enabled or not. See the docs for more detail.
