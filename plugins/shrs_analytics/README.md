
<div align="center">

# shrs_analytics


[![crates.io](https://img.shields.io/crates/v/shrs_analytics.svg)](https://crates.io/crates/shrs_analytics)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

This is a plugin for [shrs](https://github.com/MrPicklePinosaur/shrs).

## Using this plugin

First add this plugin to your dependencies
```toml
shrs_analytics = { version = "0.0.1" }
```

Then include this plugin when initializing shrs
```rust
use shrs::prelude::*;
use shrs_analytics::AnalyticsPlugin;

let myshell = ShellBuilder::default()
    .with_plugin(AnalyticsPlugin)
    .build()
    .unwrap();

```
