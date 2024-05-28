+++
title = "Builtin Commands"
description = ""
date = 2023-12-01T08:00:00+00:00
updated = 2023-12-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ''
toc = true
top = false
+++

Builtin Commands are a set of commands that users can call in the shell. **shrs** lets you create custom commands in rust and make them callable from the shell.

The main difference between builtin commands and external commands is that builtin commands have access to the shell's context during execution. This may be useful if you specifically need to query or mutate the shell's state. Some uses of this include switching the working directory, calling hooks or accessing the state store.

There is a set of predefined builtins for certain commands like `cd` and `help` in **shrs** to provide some basic functionalities. Builtins are called first after alias resolution so they will shadow other commands. You can see the available builtins by typing
```
help builtins
```

## Creating your own Builtin

An example of creating a builtin and registering it is provided below.
Builtins are simply functions that have a required parameter `&Vec<String>`.
Other states can also be accessed by adding them to the parameters; see [States](../states/).

```rust
fn my_builtin(args: &Vec<String>){
    Ok(CmdOutput::success())
}
```

Then you can register it like so
```rust
let mut builtins = Builtins::default();
builtins.insert("mybuiltin", my_builtin);

myshell.with_builtins(builtins);
```
The builtin can then be run by calling `mybuiltin`. Any existing builtins of the same name will also be overwritten, so this is a good way to override default builtins with your own version.

A much more comprehensive example can be found in the `shrs` examples directory, [here](https://github.com/MrPicklePinosaur/shrs/blob/master/crates/shrs/examples/custom_builtin.rs).

Note that we used `Builtins::default` instead of `Builtins::new`, it is highly recommended that you use the default builtins since it gives you many essential builtin commands like `cd` and `exit`, where `Builtins::new` gives you literally nothing. So it is much better practice to start with `Builtins::default` and override the ones you want.
