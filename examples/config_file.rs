//! This example shows how to write create a config file, parse it using serde and configure shrs.

use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use shrs::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    envs: Vec<(String, String)>,
    aliases: Vec<(String, String)>,
}

fn main() {
    // TODO make this relative to this file
    let config_file =
        fs::read_to_string(PathBuf::from("config.ron")).expect("Could not open config file");
    let myconfig: Config = ron::from_str(&config_file).unwrap();

    let alias = Alias::from_iter(myconfig.aliases);
    let mut env = Env::new();
    env.load();
    for (ref k, ref v) in myconfig.envs {
        env.set(k, v);
    }

    let myshell = ShellBuilder::default()
        .with_env(env)
        .with_alias(alias)
        .build()
        .unwrap();

    myshell.run();
}
