use shrs::prelude::*;
use shrs_manpages::ManPagesPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(ManPagesPlugin)
        .build()
        .unwrap();

    myshell.run();
}
