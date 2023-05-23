use shrs::prelude::*;
use shrs_mux::MuxPlugin;

fn main() {
    let myshell = ShellConfigBuilder::default()
        .with_plugin(MuxPlugin)
        .build()
        .unwrap();

    myshell.run();
}
