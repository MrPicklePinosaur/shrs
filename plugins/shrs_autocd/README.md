
<div align="center">

# shrs_autocd

implement the autocd shell option

[![crates.io](https://img.shields.io/crates/v/shrs_cd_stack.svg)](https://crates.io/crates/shrs_cd_stack)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_autocd = { version = "0.0.1" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_autocd::AutocdPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(AutocdPlugin)
    .build()
    .unwrap();

```
