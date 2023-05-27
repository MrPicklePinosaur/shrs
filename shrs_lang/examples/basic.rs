use anyhow;
use shrs_lang::{ast, eval2::eval_command, process, process::init_shell};

fn main() -> anyhow::Result<()> {
    let cmd = ast::Command::Simple {
        assigns: vec![],
        redirects: vec![],
        args: vec!["ls".into()],
    };

    init_shell()?;

    let ctx = process::Context {
        stdin: 0,
        stdout: 1,
        stderr: 2,
        is_foreground: true,
        is_interactive: true,
    };

    eval_command(&cmd, &ctx);
    Ok(())
}
