//! The shell builtin that wraps functionality of the History module

use std::{
    io::{stdout, Write},
    process::Child,
};

use crate::shell::{dummy_child, Context, Shell};

pub fn history_builtin(ctx: &mut Context, args: &Vec<String>) -> anyhow::Result<Child> {
    let history = ctx.history.all();
    for (i, h) in history.iter().enumerate() {
        print!("{} {}", i, h);
    }
    stdout().flush()?;
    dummy_child()
}
