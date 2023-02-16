#[macro_use]
extern crate lalrpop_util;

mod ast;
mod parser;
mod prompt;
mod shell;

fn main() {
    use shell::Shell;

    let mut myshell = Shell::new();
    myshell.run();
}
