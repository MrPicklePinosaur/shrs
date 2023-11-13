use shrs::prelude::*;
use shrs_mux::MuxPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(MuxPlugin::new())
        .build()
        .unwrap();

    myshell.run();
}
