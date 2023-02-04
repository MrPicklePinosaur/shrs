extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
mod runtime;
mod shell;

fn main() {
    use shell::Shell;

    let myshell = Shell::new();
    myshell.run();
}
