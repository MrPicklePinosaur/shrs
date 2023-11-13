
<div align="center">

# shrs_cd_stack

keep track of directories we have switched to in a stack fashion

[![crates.io](https://img.shields.io/crates/v/shrs_cd_stack.svg)](https://crates.io/crates/shrs_cd_stack)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_cd_stack = { version = "0.0.2" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_cd_stack::CdStackPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(CdStackPlugin)
    .build()
    .unwrap();

```
