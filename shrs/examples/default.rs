//! The most minimal working shell

use shrs::prelude::*;

fn main() {
    let myshell = ShellConfigBuilder::default().build().unwrap();

    myshell.run();
}
