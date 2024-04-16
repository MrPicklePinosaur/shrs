
<div align="center">

# shrs_rhai

run rhai scripts for shrs

[![crates.io](https://img.shields.io/crates/v/shrs_cd_stack.svg)](https://crates.io/crates/shrs_cd_stack)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_rhai = { version = "0.0.5" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_rhai::RhaiPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(RhaiPlugin)
    .build()
    .unwrap();

```
