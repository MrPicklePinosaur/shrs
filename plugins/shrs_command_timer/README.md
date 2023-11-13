
<div align="center">

# shrs_command_timer

shrs plugin to time runtime of commands

[![crates.io](https://img.shields.io/crates/v/shrs_command_timer.svg)](https://crates.io/crates/shrs_command_timer)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_command_timer = { version = "0.0.2" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_command_timer::CommandTimerPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(CommandTimerPlugin)
    .build()
    .unwrap();

```
