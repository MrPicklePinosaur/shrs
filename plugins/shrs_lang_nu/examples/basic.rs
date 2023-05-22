use shrs::prelude::*;
use shrs_lang_nu::NuLangPlugin;

fn main() {
    let myshell = ShellConfigBuilder::default()
        .with_plugin(NuLangPlugin)
        .build()
        .unwrap();

    myshell.run();
}
