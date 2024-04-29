use std::path::PathBuf;

use shrs::prelude::*;
use shrs_run_context::RunContextPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(RunContextPlugin::with_file(&PathBuf::from("./context.ron")))
        .build()
        .unwrap();

    myshell.run().expect("Error when running shell");
}
