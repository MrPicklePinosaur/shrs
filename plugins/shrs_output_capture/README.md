
<div align="center">

# shrs_output_capture

shrs plugin to capture command output

[![crates.io](https://img.shields.io/crates/v/shrs_output_capture.svg)](https://crates.io/crates/shrs_output_capture)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_output_capture = { version = "0.0.5" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_output_capture::OutputCapturePlugin;

let myshell = ShellBuilder::default()
    .with_plugin(OutputCapturePlugin)
    .build()
    .unwrap();

```
