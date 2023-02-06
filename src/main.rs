#[macro_use]
extern crate lalrpop_util;

mod ast;
mod parser;
mod shell;

fn main() {
    use shell::Shell;

    let myshell = Shell::new();
    myshell.run();
}
