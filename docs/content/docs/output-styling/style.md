+++
title = "Style"
description = ""
date = 2021-05-01T18:10:00+00:00
updated = 2021-05-01T18:10:00+00:00
draft = false
weight = 2
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
+++

Shrs uses Crossterm internally to manipulate and output to the terminal.
There are various parts of the terminal that shrs allows you to style in a custom manner, such as the prompt and output from builtins and plugins.
Styling text in shrs involves using the `StyledBuf` struct which is able to hold text where every character has its own style.

`StyledBuf` can be easily created using the `styled_buf!` macro:

```rust
styled_buf!("user".bold(),">".green())
```

The macro accepts any number of arguments. The only constraints are that the arguments must either implement the `Display` trait, be a `StyledContent` or a `StyledBuf`.
It's very easy to create styled segments using `crossterm::Stylize` which is exposed through shrs.
