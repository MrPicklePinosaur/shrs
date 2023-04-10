+++
title = "Environment Variables"
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

You can load all the current environment variables into **shrs** by using `env.load()`. This is useful in the case that you are launching your **shrs** shell from another shell, like **bash**.
```rust
let mut env = Env::new();
env.load();

myshell.with_env(env);
```

In the case that the **shrs** shell is your login shell, or that you wish to define additional environment variables, you can do so by appending to the `Env` object.
```rust
env.set("SHELL", "my_shrs");
```
