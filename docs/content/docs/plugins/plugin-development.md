+++
title = "Developing Plugins"
description = ""
date = 2021-05-01T18:10:00+00:00
updated = 2021-05-01T18:10:00+00:00
draft = false
weight = 410
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = ""
toc = true
top = false
+++

Making a plugin is as easy as implementing the `Plugin` trait. The `Plugin`
trait has an `init` method that is ran when the plugin is registered with the
`with_plugin` function. In the `init` method, you get the `shell` as context
and are free to modify it however you please, be it registering additional
hooks are adding a new builtin function.
```rust
use shrs::plugin::Plugin;

pub struct MyPlugin;

impl Plugin for PlugPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.hooks.after_command.register(after_command_hook);
        shell.builtins.insert("my_builtin", MyBuiltin::new());
        shell.state.insert(MyState::new());
    }
}

```

You can see some of the official maintained plugins for an example on how
plugins are created.
