#[macro_use]
extern crate lalrpop_util;

mod parser;
mod runtime;
mod shell;

fn main() {
    use shell::Shell;

    let myshell = Shell::new();
    myshell.run();
}
