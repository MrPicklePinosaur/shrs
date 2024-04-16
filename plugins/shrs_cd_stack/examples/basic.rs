use shrs::prelude::*;
use shrs_cd_stack::CdStackPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(CdStackPlugin)
        .build()
        .unwrap();

    myshell.run().expect("Error while running shell");
}
