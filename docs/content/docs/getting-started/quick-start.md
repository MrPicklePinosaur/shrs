+++
title = "Quick Start"
description = ""
date = 2021-05-01T08:20:00+00:00
updated = 2021-05-01T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
+++

### Prerequisites

To get started with using **shrs**, you need a functioning Rust installation.
To install Rust, you can use the [rustup](https://rustup.rs/), the Rust
toolchain installer. You will also need **cargo**, the Rust package manager.

Finally, you will need some basic knowledge on how **Rust** works, if you are
still new to **Rust**, you can consult the [rust
book](https://doc.rust-lang.org/stable/book/).

### Create Cargo project

Create your own shell project using cargo:
```sh
cargo init <project-name>
cd <project-name>
```

Next, add shrs as a dependency in your `Cargo.toml`
```toml
shrs = { version = "0.1" }
```

### Building the Shell

Next, you can create a basic shell using all of the **shrs** provided defaults with the following:
```rust
use shrs::ShellConfigBuilder;

fn main() {
    let myshell = ShellConfigBuilder::default()
        .build()
        .unwrap();

    myshell.run();
}
```

Now to run the shell
```sh
cargo run
```

From here we can start digging into all the potential configuration and
extensions that can be applied to **shrs**. See the next section for details.
