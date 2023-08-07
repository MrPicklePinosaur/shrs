+++
title = "Readline"
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

**shrs** comes with it's own readline implementation that is just as
configurable and extensible as the core.

See the section on Line Configuration for details on all the configuration
options. To pass in your own configured readline to **shrs**:
```rust
let readline = LineBuilder::default()
    .build()
    .unwrap();

myshell.with_readline(readline);
```
