use shrs::prelude::*;
use shrs_rhai_completion::CompletionsPlugin;

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(CompletionsPlugin)
        .build()
        .unwrap();

    myshell.run().unwrap();
}
