use std::{
    io::{BufRead, BufReader},
    process::{ChildStderr, ChildStdout},
};

pub fn read_out(mut reader: BufReader<&mut ChildStdout>) -> shrs::anyhow::Result<(String, i32)> {
    let mut output = String::new();
    loop {
        let mut line = String::new();

        reader.read_line(&mut line)?;

        if line.contains("\x1a") {
            let exit_status: String = line.chars().filter(|c| c.is_numeric()).collect();

            return Ok((output, exit_status.parse::<i32>()?));
        }
        print!("{}", line);

        output.push_str(line.as_str());
    }
}

pub fn read_err(mut reader: BufReader<&mut ChildStderr>) -> shrs::anyhow::Result<String> {
    let mut output = String::new();
    loop {
        let mut line = String::new();

        reader.read_line(&mut line)?;

        if line.contains("\x1a") {
            break;
        }
        eprint!("{}", line);

        output.push_str(line.as_str());
    }
    Ok(output)
}
