
# Prompt Design Document

Goal: Provide an easy and *rusty* way of building a prompt.

## TODO Design 1: Derive Macro

Inspired by clap derive builder
```
struct Prompt {
    
}
```

## Design 2: Builder Pattern

More standard rust builder pattern

Section trait, each type of section implements it. Display trait is
automatically implemented for each section.
```
trait Section {
    pub fn render(&self);
}

enum PathMode {
    Full,
    Top
}

struct Path {
    mode: PathMode
}
impl Section for Path {
    fn render(&self) {
        ...
    }
}
```

Final prompt could be built using string formatting
```
let path = Path { mode: PathMode::Top };
let username = Username::default();
let hostname = Hostname::default();
prompt!(" {username}@{hostname} {path} >");
```

or proper builder pattern
```
Prompt::builder()
    .section(Username::default())
    .text("@")
    .section(Hostname::default())
    .text(" ")
    .section(Path::top())
    .build();

// or with macro
prompt!(
    Username::default(), "@", Hostname::default(), " ", Path::top(), " > "
)
```

Considerations
- how to color sections
- conditional sections (show git dir when in repository)
    - can just return empty string

Possible sections
- Hostname
- Username
- Path (working directory)
- Indicator (vi mode, completion mode etc)
- Time
- Git info
- Project info (rust, python)
- Last command exit statu
- Time to run last command
- Custom hooks similar to [starship custom commands](https://starship.rs/config/#custom-commands)

