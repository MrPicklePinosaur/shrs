//! The most minimal working shell

use shrs::prelude::*;

fn main() {
    let myshell = ShellBuilder::default().build().unwrap();

    myshell.run().expect("Error when running shell");
}
