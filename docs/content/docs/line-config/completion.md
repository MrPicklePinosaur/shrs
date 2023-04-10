+++
title = "Completion"
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

## Default autocompletion

The default autocompletion system will autocomplete the command name using
executables in the PATH, and autocomplete all following arguments with files in
the current directory.
```rust
let completions: Vec<String> = find_executables_in_path(env.get("PATH").unwrap());
let completer = DefaultCompleter::new(completions);

myline.with_completer(completer);
```

## Coming Soon: Advanced customized completions

Upcoming is an easy to provide smart completions for a variety of programs like
`git`, `docker` and more.
