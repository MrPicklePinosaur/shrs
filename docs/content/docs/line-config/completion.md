+++
title = "Completion"
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

## Rule based

The autocompletion system works on **rules**, which are pairs of **predicates**
and **actions**. Predicates help determine when an action is allowed to run,
and actions return the actual word list for the completion system to display to
the user.

There are a variety of commonly used builtin **predicates** to make write
completions a bit easier, these include `cmdname_pred` which checks that the
current completion is for a specific command name and `flag_pred` which checks
if we are attempting to complete a flag.

Similarly, there are also builtin **actions** like `cmdname_action` which
returns a list of all executables in the PATH, and `filename_action` which
outputs all the files in the current working directory.

### Let's write completion rules for ls

As an example, let's write completion rules for the popular **ls** command.

Let's begin by initializing the `DefaultCompleter`. This comes with a couple of
sensible rules that most completion engines are expected to have, such as
autocompleting the command name from PATH:
```rust
use shrs::line::completion::*;

let mut completer = DefaultCompleter::default();
```

Next, we need to create a rule that will provide completion options for each of
the flags **ls** has. We can do this by writign a rule that first checks if the
user has already typed the command **ls** using `cmdname_pred`:
```rust
let ls_pred = Pred::new(cmdname_pred("ls"));
```

However, we also want to complete the flags for **ls**, so we need to also
check if we are currently typing a flag. We can use the provided `flag_pred`
for this. Notice how we can chain **predicates** with `.and()`:
```rust
let ls_pred = Pred::new(cmdname_pred("ls")).and(flag_pred);
```

Next we need to write the action that returns all the possible flags. An action
is just a function that takes in `CompletionCtx` and returns a list of possible
completions. `Completion` holds a bit more metadata that we will not touch for
now, but if we just wish to return a list of strings to the completer, we can
use the helper function `default_format` to generate default options for `Completion`.
```rust
let ls_action = Box::new(|ctx: &CompletionCtx| -> Vec<Completion> {
    default_format(vec!["-a".into(), "-l".into(), "-h".into()])
});
```

And with that we can register our first completion rule:
```rust
completer.register(Rule::new(ls_pred, ls_action));
```

In the end, our resulting code looks like:
```rust
use shrs::line::completion::*;

let mut completer = DefaultCompleter::default();

let ls_pred = Pred::new(cmdname_pred("ls")).and(flag_pred);
let ls_action = Box::new(|ctx: &CompletionCtx| -> Vec<Completion> {
    default_format(vec!["-a".into(), "-l".into(), "-h".into()])
});

completer.register(Rule::new(ls_pred, ls_action));
```

## Coming soon: declarative

The plugin `shrs_derive_completion` provides a declarative way to create
completions in the form of a procedual macro. If you are familiar with the
crate [clap](https://github.com/clap-rs/clap) this should feel very familiar.

