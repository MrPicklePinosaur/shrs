+++
title = "Theme"
description = ""
date = 2021-05-01T18:10:00+00:00
updated = 2021-05-01T18:10:00+00:00
draft = false
weight = 3
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
+++

Shrs provides a theme struct to allow you to change styles that are used internally such as out_style or completion_style.

```rust
myshell.with_theme(Theme {
    out_style: ContentStyle::new().red(),
    ..Default::default()
})
```
