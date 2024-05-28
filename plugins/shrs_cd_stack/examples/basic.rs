use shrs::prelude::*;
use shrs_cd_stack::{cd_stack_down, cd_stack_up, CdStackPlugin};

fn main() {
    let mut bindings = Keybindings::new();
    bindings
        .insert("C-p", "Go back in path history", cd_stack_down)
        .unwrap();
    bindings
        .insert("C-n", "Go back in path history", cd_stack_up)
        .unwrap();

    let myshell = ShellBuilder::default()
        .with_plugin(CdStackPlugin)
        .with_keybindings(bindings)
        .build()
        .unwrap();

    myshell.run().expect("Error while running shell");
}
