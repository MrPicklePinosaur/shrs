
<div align="center">

# shrs_file_logger

generate debug logs for shrs

[![crates.io](https://img.shields.io/crates/v/shrs_mux.svg)](https://crates.io/crates/shrs_file_logger)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_file_logger = { version = "0.0.3" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_file_logger::FileLogger;

let myshell = ShellBuilder::default()
    .with_plugin(FileLogger::default())
    .build()
    .unwrap();

```
