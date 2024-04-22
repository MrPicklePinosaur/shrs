//! The most minimal working shell

use shrs::prelude::*;
fn main() {
    let myshell = ShellBuilder::default().build().unwrap();
    shrs::main();

    myshell.run().expect("Error when running shell");
}
