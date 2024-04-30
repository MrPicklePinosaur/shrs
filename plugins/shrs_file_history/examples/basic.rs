//! The most minimal working shell

use shrs::shell::ShellBuilder;
use shrs_file_history::FileBackedHistoryPlugin;
fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(FileBackedHistoryPlugin {})
        .build()
        .unwrap();

    myshell.run().expect("Error when running shell");
}
