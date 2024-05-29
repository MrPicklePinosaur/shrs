+++
title = "Prompt"
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

Prompts can be customized and built with various styled custom components. Prompts are just a set of two functions that return the left and right sides of the prompt as `StyledBuf`.

First define your own prompt and implement the `Prompt` trait:
Other states can also be accessed by adding them to the parameters; see [States](../states/). Prompts return a `StyledBuf` that should be displayed.

```rust
fn prompt_left() -> StyledBuf {
    styled_buf!(
        top_pwd().blue().bold(),
        " ",
        ">".green(),
        " "
    )
}

fn prompt_right(lc: State<LineContents>) -> StyledBuf {
    styled_buf!(
        lc.cb.cursor().to_string().dark_cyan()
    )
}
```

Then add it to `ShellBuilder` when building the shell:

```rust
let shell = ShellBuilder::default()
    .with_prompt(Prompt::from_sides(prompt_left, prompt_right));
```

### Utility Functions

The `prompt` module comes with a variety of helpful functions for building the prompt. We can build something that looks like the bash prompt with:

```rust
fn prompt_left() -> StyledBuf {
    styled_buf!(hostname(),"@", username(),":", top_pwd(),"$")
}
fn prompt_right() -> StyledBuf {
    styled_buf!()
}
```
