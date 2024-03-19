
<div align="center">

# shrs_manpages

keybinding to open man page currently typed command

[![crates.io](https://img.shields.io/crates/v/shrs_manpages.svg)](https://crates.io/crates/shrs_manpages)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_manpages = { version = "0.0.4" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_manpages::ManPagesPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(ManPagesPlugin)
    .build()
    .unwrap();

```
