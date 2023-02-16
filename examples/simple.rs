use shrs::{
    prompt::{hostname, top_pwd, username},
    shell::{self, simple_error},
};

fn prompt_command() {
    use std::io::{stdout, Write};

    print!(
        "{:?}@{:?} {:?} > ",
        username().unwrap(),
        hostname().unwrap(),
        top_pwd()
    );
    stdout().flush();
}

fn main() {
    use shell::Shell;

    let mut myshell = Shell {
        prompt_command,
        error_command: simple_error,
    };
    myshell.run();
}
