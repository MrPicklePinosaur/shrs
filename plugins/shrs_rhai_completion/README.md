<div align="center">

# shrs_rhai_completion

Support for adding completions with Rhai scripts

[![crates.io](https://img.shields.io/crates/v/shrs_command_timer.svg)](https://crates.io/crates/shrs_completion)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs), which adds support for adding tab completion with [Rhai](https://github.com/rhaiscript/rhai) scripts.
Tab completions in various shells such as fish usually use scripts to handle completions and this plugin mirrors that.
For shrs, this allows scripts to be decoupled from the shell code so they can be easily modified. There will also be a set of curated Rhai scripts which can easily be copied without having to build it into the shell.
This also makes generating completions easy since other tools can be easily modified to generate Rhai scripts instead.

## Using this plugin

First add this plugin to your dependencies

```toml
shrs_rhai_completion = { version = "0.0.5" }
```

Then include this plugin when initializing shrs

Also, add completions scripts to ~/.config/shrs/completions
A list of written completions can be found in [completions](https://github.com/MrPicklePinosaur/shrs/tree/master/plugins/shrs_rhai_completion/completions)

```rust
use shrs::prelude::*;
use shrs_completion::completions::*;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(CompletionsPlugin)
        .build()
        .unwrap();

    myshell.run().unwrap();
}
```
