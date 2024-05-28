use std::{
    io::{BufRead, BufReader},
    process::{ChildStderr, ChildStdout},
};

use shrs::output_writer::OutputWriter;

pub fn read_out(
    out: &mut OutputWriter,
    mut reader: BufReader<&mut ChildStdout>,
) -> shrs::anyhow::Result<i32> {
    loop {
        let mut line = String::new();

        reader.read_line(&mut line)?;

        if line.contains("\x1a") {
            let exit_status: String = line.chars().filter(|c| c.is_numeric()).collect();

            return Ok(exit_status.parse::<i32>()?);
        }
        out.print(line)?;
    }
}

pub fn read_err(
    out: &mut OutputWriter,
    mut reader: BufReader<&mut ChildStderr>,
) -> shrs::anyhow::Result<()> {
    loop {
        let mut line = String::new();

        reader.read_line(&mut line)?;

        if line.contains("\x1a") {
            break;
        }
        out.eprint(line)?;
    }
    Ok(())
}
