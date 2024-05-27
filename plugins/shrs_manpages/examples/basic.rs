use shrs::{keybinding, prelude::*};
use shrs_manpages::open_manpage;

fn main() {
    let mut keybindings = Keybindings::new();
    keybindings.insert("C-n", "Open manpage", open_manpage);
    let myshell = ShellBuilder::default()
        .with_keybindings(keybindings)
        .build()
        .unwrap();

    myshell.run().expect("Error while running shell");
}
