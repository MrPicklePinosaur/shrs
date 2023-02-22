use shrs::{
    prompt::{hostname, top_pwd, username},
    shell::{self, simple_error, simple_exit_code},
};

fn prompt_command() {
    use std::io::{stdout, Write};

    print!(
        "{}@{} {} > ",
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
        exit_code_command: simple_exit_code,
    };
    myshell.run();
}
