
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
shrs_manpages = { version = "0.0.5" }
```

Register your own keybinding with the manpage handler
```rust
use shrs::prelude::*;
use shrs_manpages::{open_manpage};

let keybinding = keybindings! {
    |state|
    "C-n" => ("Open manpage", { open_manpage(state); }),
};

let myshell = ShellBuilder::default()
    .with_keybinding(keybinding)
    .build()
    .unwrap();

myshell.run();

```
