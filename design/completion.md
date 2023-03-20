
# Tab Completion Design Document

Goal: provide a clean API to write shell completions

Some things we should support
- look for command in path
- complete program arguments

## Background: How does bash do it?

- [Anatomy of Command Completion](https://hackaday.com/2018/01/19/linux-fu-custom-bash-command-completion/)

## Background: How does fish do it?

Fish completions are quite good

- [Fish: writing your own completions](https://fishshell.com/docs/current/completions.html)

## Design

Argument #
    - might have different completion handling for command name vs parameters

Be able to specify completions based on command name

Cursor location

What should complete command output?
- suffix of word?
- full word?

- Can provide each word, as well as current word we are on
- Can provide cursor position in word, to provide both prefix and suffix completion

prefix only completion (`|` denotes cursor position): takes only characters before cursor into account
```
input: ls|s
completions: ls|s, lsattr|s, lsb_release|s, lshw|s
```

prefix + suffix completion: takes both characters before and after cursor into account
```
// example 1
input: ls|s
completions: lsinitramfs|,  lslocks|, lsns|

// example 2
input: ls|se
completions: lsb_release|
```

Info struct passed into completion function
```rust
struct CompletionWord {
    _data: String
}

impl CompletionWord {
    fn prefix() -> String; // before cursor
    fn suffix() -> String;
}

struct Ctx {
    words: Vec<CompletionWord>
    cur: usize
}

impl Ctx {
    fn cur_word() -> CompletionWord
}
```

## Other considerations

- Investigate docopt (could make docopt first class support in shrs)
- Spell checking / spell fix?
    - Can automatically fix typos while typing?

