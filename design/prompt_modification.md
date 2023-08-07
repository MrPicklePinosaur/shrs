
# Prompt Modification Design Document

Goal: The user may wish to dynamically modify prompt contents

## Impl 1: Setter Method

Add setters and getters with custom user data and the prompt can choose what to do with it

Suggested implementation: let myprompt hold some generic user data and fetch using the type

## Impl 2: Mutation Method (don't like this very much)

Add new hook called something like 'before prompt'. It takes in the current prompt and the hook outputs a new modified prompt

```rust
// append the string hello to prompt
fn before_prompt(cur_prompt: String) -> String {
    cur_prompt.join(" hello") 
}
```
