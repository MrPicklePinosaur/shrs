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

Prompts can be customized and built with various styled custom components.

First define your own prompt and implement the `Prompt` trait:

```rust
use shrs::prelude::{Prompt, prompt::top_pwd};

struct MyPrompt;
impl Prompt for MyPrompt {
    fn prompt_left(&self, line_ctx: &LineCtx) -> StyledBuf {

        styled!(
            top_pwd().blue().bold(),
            " ",
            ">".green(),
            " "
        )
    }

    fn prompt_right(&self, line_ctx: &LineCtx) -> StyledBuf {

        styled!(
            line_ctx.cb.cursor().to_string().dark_cyan(),
        )
    }
}


impl Prompt for MyPrompt {
    fn prompt_left(&self) -> String {
        format!(" {} > ", top_pwd())
    }
}
```

Then add it to `LineBuilder` when building the shell:

```shrs

let prompt = MyPrompt;
let readline = LineBuilder::default().with_prompt(prompt);
myshell.with_readline(readline);
```

### Utility Functions

The `prompt` module comes with a variety of helpful functions for building the prompt. We can build something that looks like the bash prompt with:

```rust
struct BashPrompt;

impl Prompt for BashPrompt {
    fn prompt_left(&self, line_ctx: &LineCtx) -> StyledBuf {
        styled_buf!(hostname(),"@", username(),":", top_pwd(),"$")
    }
    fn prompt_right(&self, line_ctx: &LineCtx) -> StyledBuf {
        styled_buf!()
    }

}
```
