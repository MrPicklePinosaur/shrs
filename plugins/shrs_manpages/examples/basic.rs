use shrs::{prelude::*, keybindings};
use shrs_manpages::{ManPagesPlugin, open_manpage};

fn main() {

    let keybinding = keybindings! {
        |state|
        "C-n" => ("Open manpage", { open_manpage(state); }),
    };

    let myshell = ShellBuilder::default()
        .with_keybinding(keybinding)
        .with_plugin(ManPagesPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}
