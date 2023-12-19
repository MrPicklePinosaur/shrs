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

Builtin Commands are a set of commands that **shrs** that users can call in the shell. This lets you create custom commands in rust and make them easily available.

The main difference between builtin commands and external commands is that builtin commands
have access to the shell's context during execution. This may be useful if you specifically
need to query or mutate the shell's state. Some uses of this include switching the working
directory, calling hooks or accessing the state store.

Builtins can also be used in conjunction with plugins to easily create a command that allows users to configure plugin settings.

There is a set of predefined builtins for certain commands like `cd` and `help` in **shrs** to provide some basic functionalities.
Builtins are called first after alias resolution so they will shadow other commands.

An example of creating a builtin and registering it is provided below.

First, define a builtin and implement the `BuiltinCmd` trait:

```rust
struct EchoBuiltin;
impl BuiltinCmd for EchoBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> ::anyhow::Result<CmdOutput> {
        ctx.out.println(args.join(" "))?;
        Ok(CmdOutput::success())
    }
}
```

Then you can insert it after building the shell:

```rust
myshell.builtins.insert("echo", EchoBuiltin);
```

Now users can call echo to use the command.
