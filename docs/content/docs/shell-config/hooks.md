+++
title = "Hooks"
description = ""
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ''
toc = true
top = false
+++

Hooks are a collection of predefined functions that **shrs** will call when
certain events occur. This lets you hook onto shell events and add your own
additional functionality.

Some examples include `startup` hook, which is great for printing a welcome
message and `after_command` hook which you can use to print out the exit status
of the last command.

An example usage of the `startup` hook.
```rust
let startup_msg = | mut out: StateMut<OutputWriter>, _ctx: StartupHookCtx |
    ->Result<()>{
    out.println("Welcome to my shell!")
};
let mut hooks = Hooks::new();
hooks.insert(startup_msg);
myshell.with_hooks(hooks);
```
Hooks must have the a parameter at the end that determines which `Ctx` triggers it. They must also return a `Result<()>`.

Hooks also have additional context that is passed as a parameter which you can
leverage. For a list of all the hooks and the context that is passed, please
refer to the rust docs.
