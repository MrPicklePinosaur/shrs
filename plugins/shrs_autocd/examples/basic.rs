use shrs::prelude::*;
use shrs_autocd::AutocdPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(AutocdPlugin)
        .build()
        .unwrap();

    myshell.run();
}
