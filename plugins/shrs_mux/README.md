
<div align="center">

# shrs_mux

switch command language at runtime

[![crates.io](https://img.shields.io/crates/v/shrs_mux.svg)](https://crates.io/crates/shrs_mux)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_mux = { version = "0.0.3" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_mux::MuxPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(MuxPlugin::new())
    .build()
    .unwrap();

```
