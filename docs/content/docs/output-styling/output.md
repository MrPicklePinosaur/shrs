+++
title = "Output"
description = ""
date = 2021-05-01T18:10:00+00:00
updated = 2021-05-01T18:10:00+00:00
draft = false
weight = 1
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
+++

Printing to stdout can be done in shrs using `println!()`, however it is recommended to use `ctx.out` whenever possible.
It provides various print functions that will automatically use the configured out and error colors. Output is also recorded
and can be accessed by hooks through the AfterCommandCtx. `print_buf` is also provided to allow users to easily output `StyledBuf`.

```rust
ctx.out.println("Hello")?;
ctx.out.print_buf(styled_buf!("Hello".red()))?;
```
