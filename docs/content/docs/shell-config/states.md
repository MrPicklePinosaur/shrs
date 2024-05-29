+++
title = "States"
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

**shrs** is highly extensible and allows customizing the states of the different parts of the shell. Shrs also allows users to define their own states which can be used to control various behaviours.

## Accessing states
States can be accessed in various defined callbacks such as [builtins](../builtins/), [hooks](../hooks/), [keybindings](../keybindings/), [prompt](../prompt/) and when initializing a plugin.

States are accessed by adding it to the function parameters, wrapped in either `State` or `StateMut`, depending on if the state should be mutable or not. Accessing `OutputWriter` in a keybinding to write to console can be done like so.

```rust
fn clear_screen(mut out: StateMut<OutputWriter>)-> {
    out.println("Clear Screen")?;
}
```

Parameters can also be wrapped in an Option, if the state may not exist when the function is called. Otherwise, **shrs** will panic.

```rust
fn clear_screen(mut out: Option<StateMut<OutputWriter>>)-> {
    if let Some(o) = out{
        out.println("Clear Screen")?;
    }
}
```

Shell is a special state that can only be accessed immutably and is guaranteed to always exist. Accessing state does not require `State` or `StateMut`.

```rust
fn clear_screen(mut out: Option<StateMut<OutputWriter>>,sh: &Shell)-> {
    out.println("Clear Screen")?;
}
```

## Defining custom states
Custom states allow users to create states that can be accessed in the same manner as above. States can be easily inserted before the shell starts.

```rust
pub struct T{}
fn main(){
    let myshell = ShellBuilder::default().with_state(T{}).build.unwrap();
    myshell.run().unwrap();
}

```
States can also be queued to be inserted during callbacks and are inserted directly after.
```rust
pub struct H{}

pub fn f(sh: &Shell, ctx: &SCtx) -> Result<()> {
    sh.run_cmd(|sh: &mut Shell, states: &mut States| {
        states.insert(H{});
    });

    Ok(())
}
```
