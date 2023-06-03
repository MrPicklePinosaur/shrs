+++
title = "Using Plugins"
description = ""
date = 2021-05-01T18:10:00+00:00
updated = 2021-05-01T18:10:00+00:00
draft = false
weight = 409
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
+++

Plugins are just regular crates that can be obtained from
[crates.io](https://crates.io/). Most **shrs** related crates have the prefix
`shrs_`. Simply add the crate to your project. To make **shrs** use the plugin,
it's as using as using `with_plugin` when constructing the shell and pass in
the plugin.
```rust
let myshell = ShellBuilder::default()
    .with_plugin(OutputCapturePlugin)
    .build()
    .unwrap();

myshell.run();
```
