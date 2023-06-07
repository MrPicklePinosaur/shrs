
<div align="center">

# sh.rs

The rusty POSIX shell library for hackers

[![crates.io](https://img.shields.io/crates/v/shrs.svg)](https://crates.io/crates/shrs)
[![Doc](https://docs.rs/shrs/badge.svg)](https://docs.rs/shrs)
[![book](https://img.shields.io/badge/book-website-orange)](https://mrpicklepinosaur.github.io/shrs/docs/getting-started/introduction/)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)
[![Discord](https://img.shields.io/discord/1109393362626297856?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/XYeMseQrbM)

</div>

## PROJECT GOALS

- **hackable**: almost every aspect of the shell should be easily customizable and extendable
- **developer friendly**: well documented and easy to contribute to
- **rusty**: implemented in and configurable in rust

## FEATURES

<img width=50% src="media/demo.gif"/>

DISCLAIMER: **shrs** is currently very much a work in progress, the API is subject to change frequently and things are prone to breaking. It currently may not be suitable for daily use, but is great for prototyping any experimental shell features you dream up with!

Here are what makes **shrs** special:
- Completely configurable in rust (including your prompt, completions and more!)
- Plugin system (community maintained plugins that add unique features)

## GETTING STARTED

To get a taste of what **shrs** is capable of, without any configuration, you can run the example **shrs_example** shell that is bundled by running:
```sh
git clone https://github.com/MrPicklePinosaur/shrs.git
cd ./shrs/shrs_example
cargo run
```

To get started with building your own shell, it's as easy as creating a new cargo project with:
```rust
use shrs::prelude::*;

fn main() {
    let myshell = ShellBuilder::default()
        .build()
        .unwrap();

    myshell.run();
}
```

See the [developer documentation](https://mrpicklepinosaur.github.io/shrs/docs/getting-started/introduction/) for more in depth information.

## CONTRIBUTING

If you encounter and bugs are have any feature requests, please don't hesitate to [leave an issue](https://github.com/MrPicklePinosaur/shrs/issues)! Also take a look at the section for contributors in the [documentation](https://mrpicklepinosaur.github.io/shrs/docs/contributing/how-to-contribute/).

