use shrs::prelude::*;
use shrs_rhai::RhaiPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(RhaiPlugin)
        .build()
        .unwrap();

    myshell.run();
}
