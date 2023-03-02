use std::default;

use shrs::{
    alias::Alias,
    builtin::Builtins,
    prompt::{hostname, top_pwd, username},
    shell::{self, Context, Runtime},
};

fn main() {
    use shell::Shell;

    let myshell = Shell {
        ..Default::default()
    };

    let mut alias = Alias::new();
    alias.set("ls", "ls -al");
    let mut ctx = Context {
        alias,
        ..Default::default()
    };
    let mut rt = Runtime {
        ..Default::default()
    };
    myshell.run(&mut ctx, &mut rt);
}
