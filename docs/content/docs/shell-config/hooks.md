+++
title = "Hooks"
description = ""
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
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
let startup_msg = | _sh: &Shell, _sh_ctx: &mut Context, _sh_rt: &mut Runtime, _ctx: StartupHookCtx | {
    let welcome_str = "Welcome to my shell!";
    println!("{}", welcome_str);
};
let hooks = Hooks {
    startup: HookList::from_iter(vec![startup_msg]),
    ..Default::default()
};

myshell.with_hooks(hooks);
```

Hooks also have additional context that is passed as a parameter which you can
leverage. For a list of all the hooks and the context that is passed, please
refer to the rust docs.

Also notice that each type of hook actually takes in a list of hooks to run.
These hooks are ran in the order they are registered.
