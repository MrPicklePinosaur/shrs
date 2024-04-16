use std::{
    io::{BufRead, BufReader},
    process::{ChildStderr, ChildStdout},
};

use shrs::prelude::Context;

pub fn read_out(
    ctx: &mut Context,
    mut reader: BufReader<&mut ChildStdout>,
) -> shrs::anyhow::Result<i32> {
    loop {
        let mut line = String::new();

        reader.read_line(&mut line)?;

        if line.contains("\x1a") {
            let exit_status: String = line.chars().filter(|c| c.is_numeric()).collect();

            return Ok(exit_status.parse::<i32>()?);
        }
        ctx.out.print(line)?;
    }
}

pub fn read_err(
    ctx: &mut Context,
    mut reader: BufReader<&mut ChildStderr>,
) -> shrs::anyhow::Result<()> {
    loop {
        let mut line = String::new();

        reader.read_line(&mut line)?;

        if line.contains("\x1a") {
            break;
        }
        ctx.out.eprint(line)?;
    }
    Ok(())
}
