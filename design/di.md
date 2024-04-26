Make Keybindings, Highlighter, Lang, Prompt and Builtins DI.

- Can use builder pattern where necessary
- Make it so that everything is available except for itself
- Make the running a method of the thing itself, so change

## `ctx.run_hooks();` to `ctx.hooks.run();`

`Builtins::insert(name, ||->CmdOutput);`

maybe this will have an intermediary BuiltinCmd struct to store metadata

`Prompt::from(||->StyledBuf, ||->StyledBuf)`

This will require a specialized DI for each type, how to standardize and reuse code

Make things in Context that are not in state available in State<T>,
maybe add values to temp dictionary and use that.

Or make the reference in state the same somehow (idk how)

Everything moves to Context,
No More LineBuilder move all values to ShellBuilder

# NEXT DAY REWRITE

Shell is back as a struct that's passed around

All things that need to be run and passed Ctx to run are stored in here
such as hooks, lang etc.

Separation of church and state - This is not necessary since I'm still passing &sh as immutable. Idea is to mutate sh need to use a command queue. Like bevy does. The changes will be applied at the end of the loop

hooks is now immutable

```rust
pub trait Command: Send + 'static {
    /// Applies this command, causing it to mutate the provided `world`.
    ///
    /// This method is used to define what a command "does" when it is ultimately applied.
    /// Because this method takes `self`, you can store data or settings on the type that implements this trait.
    /// This data is set by the system or other source of the command, and then ultimately read in this method.
    fn apply(self, sh: &mut sh);
}
```

made hooks immutable -> maybe open pr for book?

WAIT
How do i modify State, Do i need to?

Solution: Bevy Command Queue

# Specialization Problem

```rust

impl<F, C: Ctx> IntoHook<(), C> for F
where
    for<'a, 'b> &'a F: Fn(&C) -> Result<()>,
{
    type Hook = FunctionHook<C, Self>;

    fn into_system(self) -> Self::Hook {
        FunctionHook {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<F, C: Ctx> IntoHook<(), C> for F
where
    for<'a, 'b> &'a F: Fn(&Shell, &C) -> Result<()>,
{
    type Hook = FunctionHook<(Shell, C), Self>;

    fn into_system(self) -> Self::Hook {
        FunctionHook {
            f: self,
            marker: Default::default(),
        }
    }
}
```

Need to implement IntoHook for both somehow, Specialization is unstable TODO other solutions.

For Now: Shell is mandatory TODO put up issue

# Another Iteration

Move everything in Context to States
Use States for everything
Problems: Interior Mutability leads to runtime issues
Easy to debug with rust_backtrace
Current problem with completions borrow mut error

Can't Query for Command in hooks being run in commands
Can't use sh.run_hooks() forced to use sh.hooks.run
