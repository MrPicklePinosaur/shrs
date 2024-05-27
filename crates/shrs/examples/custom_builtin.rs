//! This example shows how to create your own custom builtin and register it
//!
//! The example builtin will be a 'better cd' plugin, which onto of acting like normal cd, also
//! allows you to provide alias specific directories using `cd @work` syntax

use std::{collections::HashMap, path::PathBuf};

use shrs::prelude::*;
use shrs_core::shell::set_working_dir;

fn hello_builtin(
    mut out: StateMut<OutputWriter>,
    sh: &Shell,
    args: &Vec<String>,
) -> ::anyhow::Result<CmdOutput> {
    if let Some(s) = args.first() {
        out.println(format!("Hello {s}"));
    } else {
        out.println("Hello World");
    }
    Ok(CmdOutput::success())
}

fn main() {
    // use Builtins::default() since it gives us some default builtins to use, rather than
    // Builtins::new() which gives us nothing
    let mut builtins = Builtins::default();

    // register our custom builtin to override the default cd
    builtins.insert("hello", hello_builtin);

    let myshell = ShellBuilder::default()
        .with_builtins(builtins)
        .build()
        .unwrap();

    myshell.run().unwrap();
}
