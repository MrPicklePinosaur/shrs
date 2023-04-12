//! Capture stdout and stderr of previous command outputs
//!
//!

use std::{io::BufWriter, marker::PhantomData};

use shrs::{anyhow, hooks::AfterCommandCtx};

pub fn after_command_hook(
    out: &mut BufWriter<std::io::Stdout>,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    Ok(())
}
