use std::{env, path::Path};

use shrs::prelude::*;
use shrs_mux::{python::*, BashLang, MuxPlugin, NuLang, SqliteLang, SshLang};

fn main() {
    // the remote supplied here currently does not support interactive passwords, use an ssh key
    // without a password for now :((
    // let ssh_remote = env::var("SHRS_SSH_ADDRESS").unwrap();

    let mux_plugin = MuxPlugin::new()
        .register_lang("bash", BashLang::new())
        .register_lang("python", PythonLang::new())
        .register_lang("nu", NuLang::new())
        // .register_lang("ssh", SshLang::new(ssh_remote))
        .register_lang("sqlite", SqliteLang::new(Path::new("/tmp/test.sqlite")));

    let myshell = ShellBuilder::default()
        .with_plugin(mux_plugin)
        .build()
        .unwrap();

    myshell.run().unwrap();
}
