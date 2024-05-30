---
title: "Implementing Dependency Injection in rust"
url: "/blog/doks-1/"
description: "Improving library ergonomics using dependency injection"
summary: "the development process behind implementing dependency injection in rust, and its advantages"
date: 2024-05-28
lastmod: 2024-05-28
draft: false
weight: 50
categories: []
tags: []
pinned: false
homepage: false
extra:
  authors:
    - name: "Nithin"
      href: "https://github.com/nithinmuthukumar"

---
This blog post is about how I developed DI injection in rust and how it solved various challenges for [shrs](https://github.com/MrPicklePinosaur/shrs). Shrs is designed to have an API that allows access various shell states (e.g. Aliases, Keybindings, Line content) at various points in the shell. Users are able to write their own prompts, highlighters, history implementations as callbacks, which have access to a shared state. The most direct solution to this is using a global mutable state, however that has many issues, especially as a library. As a consequence, we had to pass around mutable references to state everywhere, making the traits in the user facing API very verbose:
```rust
// Implementing this trait allows users to write their own shell builtin commands
// sh, ctx and rt represent global state
pub trait BuiltinCmd {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<CmdOutput>;
}
```
User defined states were managed in an `AnyMap`. Accessing these was also verbose. These problems have been plaguing shrs since the start and I thought it was about time I tried to fix them.

## SOLS
Having prior experience with Bevy, I found the idea of implementing a similar system in SHRS a great idea. Imagine having callbacks that resemble systems, with variable parameters and the ability to access requested resources. The design paradigm I was looking for was Dependency Injection.

After searching around I found this tutorial [Dependency Injection like Bevy Engine from Scratch](https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/introductions.html). I was able to quickly adapt the provided code to fit shrs. It instantly felt like a great improvement. For example, writing a builtin went from [this](https://github.com/MrPicklePinosaur/shrs/blob/e2f839806d37108120394e3e5cdfad495ce2701c/crates/shrs_core/src/builtin/help.rs) to [this](https://github.com/MrPicklePinosaur/shrs/blob/master/crates/shrs_core/src/builtin/help.rs).

Users no longer needed to create a struct for a single method and the user only needed to have params for what they needed.

State was still broken into two pieces, since Shell had to be immutable. Shell stores the systems that are run and its impossible to pass a mutable reference of Shell to the systems. This led to many limitations where it was impossible to modify anything in Shell. Bevy faces a similar problem with inserting resources at runtime, and I copied their [method](). Basically, inside of a system users are able to queue up changes that will occur directly after the callback ends. This means that the mutable reference will be dropped and Shell is modifiable.

## Optional Parameters

## Lingering problems
