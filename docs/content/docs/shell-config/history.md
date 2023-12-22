+++
title = "History"
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

History is the ability for the shell to remember previous commands you have
typed, providing the ability to cycle back to re-run commands. Currently
**shrs** offers two history implementations, `DefaultHistory` and
`FileBackedHistory`. `DefaultHistory` is a very basic memory based history,
which means that your history will not persist if you close the shell. On the
other hand, `FileBackedHistory` uses an actual file on disk, providing
persistent completions.

Here is an example of using `FileBackedHistory`:
```rust
// Put the path to your history file here
let history_file = PathBuf::from(" ... ");
let history = FileBackedHistory::new(history_file).unwrap();

myshell.with_history(history)
```
