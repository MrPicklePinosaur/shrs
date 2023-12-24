+++
title = "Keybindings"
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

Keybindings allow you to run arbitrary commands in respond to arbitrary key
presses in the shell. A common example is the ability to clear the terminal
when `Control+L` is pressed. How keybindings are represented is a bit more of an
internal detail and not very fun to write, so a macro is provided to make the
experience a bit better.
```rust
let keybinding = keybindings! {
    "C-l" => Command::new("clear").spawn(),
};

myshell.with_keybinding(keybinding);
```

The macro allows us to represent key combinations in terms of strings. You can
also include modifier keys (such as control and shift). Here are the supported modifiers:

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
