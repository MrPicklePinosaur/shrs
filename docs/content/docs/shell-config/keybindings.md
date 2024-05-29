+++
title = "Keybindings"
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
Keybindings allow you to run arbitrary commands in respond to arbitrary key
presses in the shell. A common example is the ability to clear the terminal
when `Control+L` is pressed. Keybindings are also able to access `State` (see [States](../states/)), which could be used to modify on keypress.
```rust

let mut bindings = Keybindings::new();
bindings
    .insert("C-l", "Clear the screen", || -> anyhow::Result<()> {
        Command::new("clear")
            .spawn()
            .expect("Couldn't clear screen");
        Ok(())
    })
    .unwrap();
//cd_stack_down and cd_stack_up are hook functions
bindings
    .insert("C-p", "Move up one in the command history", cd_stack_down)
    .unwrap();
bindings
    .insert("C-n", "Move down one in the command history", cd_stack_up)
    .unwrap();


myshell.with_keybindings(bindings);
```

Each keybinding is inserted as a tuple. The first item in the tuple is the required key combination. The second provides info on what the binding does. The third item is the function that will be executed once that key combination is pressed.

key combinations are represented in terms of strings and stored internally as a `KeyEvent`. You can also include modifier keys (such as control and shift). Here are the supported modifiers:

| Modifier | Usage |
| ---|--- |
| Shift | `"S"` or `"Shift"` |
| Alt | `"A"` or `"Alt"` |
| Control | `"C"` or `"Ctrl"` |
| Super | `"Super"` |
| Meta | `"M"` or `"Meta"` |

Furthermore, there are also some keys that are hard to represent in a string,
so we use a special notation to denote them:

| Key | Usage |
| ---|--- |
| Space | `"<space>"` |
| Backspace | `"<backspace>"` |
| Escape | `"<esc>"` |
| Enter | `"<enter>"` |
| Tab | `"<tab>"` |

Here are some example keybinding strings:

| Meaning | Usage |
| ---|--- |
| Control + Alt + q | `"C-A-q"` |
| Super + Space | `"Super-<space>"` |
| Alt + Tab | `"A-<tab>"` |
