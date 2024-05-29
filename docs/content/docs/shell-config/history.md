+++
title = "History"
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

History is the ability for the shell to remember previous commands you have
typed, providing the ability to cycle back to re-run commands.
**shrs** provides `DefaultHistory` which is a very basic memory based history,
which means that your history will not persist if you close the shell. On the
other hand, `FileBackedHistoryPlugin` from shrs_file_history provides `FileBackedHistory` which writes to a file on disk, providing
persistent completions.

To add file backed history to shrs, simply add the plugin to shell:
```rust
// The file defaults to ~/.config/shrs/history
myshell.with_plugin(FileBackedHistoryPlugin::new());
```
