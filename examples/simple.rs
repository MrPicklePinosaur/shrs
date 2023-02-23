use shrs::{
    alias::Alias,
    builtin::Builtins,
    prompt::{hostname, top_pwd, username},
    shell::{self, simple_error, simple_exit_code, Context},
};

fn prompt_command() {
    use std::io::{stdout, Write};

    print!(
        "{}@{} {} > ",
        username().unwrap(),
        hostname().unwrap(),
        top_pwd()
    );
    stdout().flush();
}

fn main() {
    use shell::{Hooks, Shell};

    let hooks = Hooks {
        prompt_command,
        ..Default::default()
    };
    let builtins = Builtins::default();
    let myshell = Shell::new(hooks, builtins);

    let mut alias = Alias::new();
    alias.set("ls", "ls -al");
    let mut ctx = Context {
        alias,
        ..Default::default()
    };
    myshell.run(&mut ctx);
}
