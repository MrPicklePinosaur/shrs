use shrs::prelude::*;
use shrs_mux::{BashLang, MuxPlugin, NuLang, PythonLang};

fn main() {
    let mux_plugin = MuxPlugin::new()
        .register_lang("bash", BashLang::new())
        .register_lang("python", PythonLang::new())
        .register_lang("nu", NuLang::new());

    let myshell = ShellBuilder::default()
        .with_plugin(mux_plugin)
        .build()
        .unwrap();

    myshell.run().unwrap();
}
