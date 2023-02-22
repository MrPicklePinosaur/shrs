use shrs::{
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
    let mut ctx = Context::new();
    myshell.run(&mut ctx);
}
