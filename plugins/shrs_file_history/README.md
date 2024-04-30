<div align="center">

# shrs_file_history

file backed history
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>
This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies

```toml
shrs_command_timer = { version = "0.0.5" }
```

Then include this plugin when initializing shrs

```rust
use shrs::shell::ShellBuilder;
use shrs_file_history::FileBackedHistoryPlugin;
fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(FileBackedHistoryPlugin::new())
        .build()
        .unwrap();

    myshell.run().expect("Error when running shell");
}
```
