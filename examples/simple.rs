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
    use shell::{Hooks, Shell};

    let hooks = Hooks {
        prompt_command,
        ..Default::default()
    };
    let mut myshell = Shell::new(hooks);
    myshell.run();
}
