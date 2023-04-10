+++
title = "Aliases"
description = ""
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ''
toc = true
top = false
+++

Aliases can be specified as a key value pair of the alias name and the actual command it expands to. Keep in mind that aliases are not evaluated or syntax checked at time of definition, only during substitution. This means that it is possible to define aliases that are invalid commands.
```rust
let alias = Alias::from_iter([
    ("l".into(), "ls".into()),
    ("c".into(), "cd".into()),
    ("g".into(), "git".into()),
    ("v".into(), "vim".into()),
    ("la".into(), "ls -a".into()),
]);

myshell.with_alias(alias);
```
