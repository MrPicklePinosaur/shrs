//! This example shows how to write create a config file, parse it using serde and configure shrs.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use shrs::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    envs: Vec<(String, String)>,
    aliases: Vec<(String, String)>,
}

fn main() {
    // TODO make this relative to this file
    let config_file = fs::read_to_string(PathBuf::from("./config.ron")).unwrap();
    let myconfig: Config = ron::from_str(&config_file).unwrap();

    let alias = Alias::from_iter(myconfig.aliases);

    let myshell = ShellConfigBuilder::default()
        .with_alias(alias)
        .build()
        .unwrap();

    myshell.run();
}
