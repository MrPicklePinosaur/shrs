+++
title = "Snippets"
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

Snippets are substitutions that apply in the line when a trigger key is pressed.
When a given snippet is triggered it will expand based on a behaviour to another string.
```rust
let mut snippets = Snippets::new(ExpandSnippet::OnSpace);
snippets.add(
    "gc".to_string(),
    SnippetInfo::new("git commit -m \"", Position::Command),
);
snippets.add(
    "ga".to_string(),
    SnippetInfo::new("git add .", Position::Command),
);
let shell = ShellBuilder::default().with_snippets(snippets).unwrap();
shell.run().unwrap()

```
