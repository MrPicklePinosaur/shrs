
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

## Background: How does nushell do it?

[Nushell custom completions](https://www.nushell.sh/book/custom_completions.html#context-aware-custom-completions)

## Requirements

- custom completion handlers depending on type of argument
    - argument, subcommand, git branch, ssh host etc
- 

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

## Completion specification

We also need a way to specify how certain commands should be expanded.

Currently, the way completions are written by other shells is not the greatest, take fish for example:
```fish
complete -c timedatectl -f

complete -c timedatectl -n "not __fish_seen_subcommand_from $commands" \
    -a "status set-time set-timezone list-timezones"

complete -c timedatectl -n "__fish_seen_subcommand_from set-timezone" \
    -a "(timedatectl list-timezones)"
```

Instead, we can use a derive macro:
```rust
#[derive(Completion)]
struct GitCompletion {

    #[arg(short, long, description = "display verbose information")]
    verbose: bool,

    #[command(subcommand)]
    commands: Option<Commands>

}

#[derive(Subcommand)]
enum Commands {
    Add {

    },
    Status {

    }
}
```
also consider if we can leverage the derive macro from `clap` since this seems quite similar.

The underlying struct the Completion macro should generate should look something like this:
```rust
struct CompletionInfo {
    cmd_name: String,
    subcommands: HashMap<String, Subcommand>,
    args: Vec<Arg>
}

struct Subcommand {
    args: Vec<Arg>,
}

struct Arg<T> {
    short: Option<char>,
    long: String,
    description: Option<String>
    parameter: T,
}
```

And when calling, need something like:
```rust
let comp = CompletionInfo::new();
comp.completions_from("git status");
```

The completion needs to return something like a reference to a
'half-traversed' structure of a command, we are pointing to a specific
location in the tree that defines the command grammar and need figure out where
we can go next.

## Other considerations

- Investigate docopt (could make docopt first class support in shrs - or just a plugin)
- Spell checking / spell fix?
    - Can automatically fix typos while typing?

## Resources
- [Pragmatic approach to shell completion](https://dev.to/rsteube/a-pragmatic-approach-to-shell-completion-4gp0)
- [carapace](https://github.com/rsteube/carapace)
- [carapace-bin](https://github.com/rsteube/carapace-bin)
- [zsh completion system explanation](https://zsh.sourceforge.io/Guide/zshguide06.html)
