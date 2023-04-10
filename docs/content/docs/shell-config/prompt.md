+++
title = "Prompt"
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

First define your own prompt and implement the `Prompt` trait.
```rust
use shrs::{Prompt, prompt::top_pwd};

struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self) -> String {
        format!(" {} > ", top_pwd())
    }
}
```

Then add it when building the shell:
```shrs
let prompt = MyPrompt;

myshell.with_prompt(prompt);
```

### Utility Functions

The `prompt` module comes with a variety of helpful functions for building the prompt. We can build a something that looks like the bash prompt with:
```rust
struct BashPrompt;

impl Prompt for BashPrompt {
    fn prompt_left(&self) -> String {
        format!("{}@{}:{}$ ", hostname(), username(), top_pwd())
    }
}
```
