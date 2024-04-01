use shrs::{prelude::*, keybindings};
use shrs_manpages::{open_manpage};

fn main() {

    let keybinding = keybindings! {
        |state|
        "C-n" => ("Open manpage", { open_manpage(state); }),
    };

    let myshell = ShellBuilder::default()
        .with_keybinding(keybinding)
        .build()
        .unwrap();

    myshell.run();
}
