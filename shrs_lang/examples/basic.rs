use anyhow;
use shrs_lang::{
    ast,
    eval2::eval_command,
    process::{self, Os},
};

fn main() -> anyhow::Result<()> {
    let mut os = Os::new();
    os.init_shell()?;

    let inner_cmd = ast::Command::Pipeline(vec![
        Box::new(ast::Command::Simple {
            assigns: vec![],
            redirects: vec![],
            args: vec!["echo".into(), "poo".into()],
        }),
        Box::new(ast::Command::Simple {
            assigns: vec![],
            redirects: vec![],
            args: vec!["tr".into(), "o".into(), "e".into()],
        }),
    ]);
    let cmd = ast::Command::AsyncList(
        Box::new(inner_cmd),
        Some(Box::new(ast::Command::Simple {
            assigns: vec![],
            redirects: vec![],
            args: vec!["echo".into(), "poo".into()],
        })),
    );

    let ctx = process::Context {
        stdin: 0,
        stdout: 1,
        stderr: 2,
        is_foreground: true,
        is_interactive: true,
    };

    let res = eval_command(&mut os, &cmd, &ctx)?;
    match res {
        process::ExitStatus::Exited(status) => println!("exited {status}"),
        process::ExitStatus::Running(pid) => {},
    }

    Ok(())
}
