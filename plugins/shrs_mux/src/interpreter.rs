use std::{
    io::{BufRead, BufReader},
    process::{Child, ChildStderr, ChildStdout},
    sync::{Arc, Mutex},
};

pub fn read_out(instance: Arc<Mutex<Child>>) -> shrs::anyhow::Result<(String, i32)> {
    let mut output = String::new();
    let exit_status: i32;
    loop {
        let mut guard = instance.lock().unwrap();
        let mut reader = BufReader::new(guard.stderr.as_mut().expect("Failed to open stderr"));

        let mut line = String::new();

        reader.read_line(&mut line)?;
        println!("vyv{}", line.clone());

        if line.contains("\x1a") {
            let s: String = line.chars().filter(|c| c.is_numeric()).collect();
            exit_status = s.parse::<i32>()?;
            break;
        }

        output.push_str(line.as_str());
    }
    Ok((output, exit_status))
}

pub fn read_err(instance: Arc<Mutex<Child>>) -> shrs::anyhow::Result<String> {
    let mut output = String::new();
    loop {
        let mut guard = instance.lock().unwrap();

        let mut reader = BufReader::new(guard.stderr.as_mut().expect("Failed to open stderr"));

        let mut line = String::new();

        reader.read_line(&mut line)?;

        println!("err{}", line.clone());

        if line.contains("\x1a") {
            break;
        }

        output.push_str(line.as_str());
    }
    Ok(output)
}
