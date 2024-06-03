
<div align="center">

# shrs_completion

more completions for shrs

[![crates.io](https://img.shields.io/crates/v/shrs_command_timer.svg)](https://crates.io/crates/shrs_completion)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_completion = { version = "0.0.6" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_completion::completions::*;

fn main() {
    let mut mycompleter = DefaultCompleter::default();
    ssh_completion(&mut mycompleter);

    let myline = LineBuilder::default()
        .with_completer(mycompleter)
        .build()
        .unwrap();

    let myshell = ShellBuilder::default()
        .with_readline(myline)
        .build()
        .unwrap();

    myshell.run().unwrap();
}
```
