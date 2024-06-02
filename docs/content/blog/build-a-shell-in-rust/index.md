---
title: "Build a unix shell in Rust using shrs"
url: "/blog/build-a-shell-in-rust/"
description: "Learn how you can build your own unix shell in minutes using shrs - the rusty shell toolkit"
summary: ""
date: 2024-06-01
lastmod: 2024-06-01
draft: false
weight: 50
categories: []
tags: []
pinned: false
homepage: false
extra:
  authors:
    - name: "pinosaur"
      href: "https://github.com/MrPicklePinosaur"
---

## What even is a shell?

At it's most basic, a [shell](https://en.wikipedia.org/wiki/Unix_shell) is a program that executes other programs. It provides an interface using text commands from the user written in the shell's interpreted language, and may support features like scripting, intelligent tab completion, command history, and more. In the Linux world, there are a number of options for shells you can use, the most popular ones being [bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) (installed by default on most Linux distributions) and [zsh](https://en.wikipedia.org/wiki/Z_shell) (installed by default on MacOS). However, these shells have existed for more than thirty years, carrying much historical baggage. We have seen the emergence of modern shells, with novel ideas and new paradigms, notably [nushell](https://www.nushell.sh/) - with structured data and written in Rust, as well as [fish](https://fishshell.com/) - prioritizing user friendliness and delightful completions.

## So what's shrs?

[shrs](https://github.com/MrPicklePinosaur/shrs) isn't just another shell that's written in Rust, it's a toolkit for you to create your own shell! It can also be thought of as a shell whose _configuration language_ is Rust. **shrs** gives you a framework to build a shell exactly how you like it, providing an order of magnitude more control than existing shell configuration.

## Getting started

The pseudocode below provides a highly simplified view on how a shell functions.
```
loop {
    input = get_user_input()
    command = parse_command(input)
    run_command(command)
}
```
In short, it runs an infinite loop that 
1) _Reads a line of input from the user_ - while reading this line, the shell can provide features to make the user's life easier, things like tab completion, vi mode, history, syntax highlighting, and much more. Once the user presses `Enter`, the command is accepted
2) _Parsed by the shell interpreter_ - shells provide a 'programming language' that is able to execute complex user commands, which means shells need to implement an interpreter akin to other interpreted languages - like python
3) _Command execution_ - after the shell has converted the text input into a format it can understand, it asks the operating system to do what the user requested; this may be spawning a program, opening a file or setting up pipes for processes to talk to each other

<!-- Traditionally, this job is done by the [GNU readline](https://en.wikipedia.org/wiki/GNU_Readline) program, but **shrs** provides it's own configurable version implemented in Rust. -->

**shrs** provides a framework that handles the heavy lifting of the core shell logic and lets you focus on the components that matter. In fact, you can produce a working shell in just a couple lines of code! Let's get started by creating a new Rust project. If you don't have Rust set up already, [install](https://www.rust-lang.org/tools/install) it using rustup (recommended). With Rust installed, let's create a new project using cargo
```sh
cargo new --bin <project-name>
cd <project-name>
```
First we need to add **shrs** as a dependency in your `Cargo.toml` file
```toml
[dependencies]
shrs = { version = "0.0.6" }
```
Then in `main.rs`, we can create the most basic **shrs** shell
```rust
use shrs::prelude::*;

fn main() {
    let myshell = ShellBuilder::default()
        .build()
        .unwrap();

    myshell.run();
}
```
Breaking down each line:
- `use shrs::prelude::*` - imports most of the functions, traits and modules that you will need
- `ShellBuilder::default()` - creates a new shell configuration, this is the one-stop shop for all config options. See the [docs](https://docs.rs/shrs/latest/shrs/prelude/struct.ShellBuilder.html) for more info. Once we are happy with the options, we call `build()` to finalize the configuration and produce a runnable shell.
- `myshell.run()` - start running the shell loop

And that's it! We have a working shell in just a couple of lines of code!

// TODO insert gif of basic shell

 The default shell with no configuration options is quite bare bones however.

