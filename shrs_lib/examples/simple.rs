use std::default;

use shrs::{
    alias::Alias,
    builtin::Builtins,
    env::Env,
    prompt::{hostname, top_pwd, username},
    shell::{self, Context, Runtime},
};
use shrs_line::line::Line;

fn main() {
    use shell::Shell;

    let completions: Vec<String> = find_executables_in_path(rt.env.get("PATH").unwrap());
    let completer = shrs_line::completion::DefaultCompleter::new(completions);
    let menu = shrs_line::menu::DefaultMenu::new();
    let history = shrs_line::history::DefaultHistory::new();
    let cursor = shrs_line::cursor::DefaultCursor::default();

    let readline = Line::new(menu, completer, history, cursor);

    let myshell = Shell {
        ..Default::default()
    };

    let mut alias = Alias::new();
    alias.set("ls", "ls -al");
    let mut ctx = Context {
        readline,
        alias,
        ..Default::default()
    };
    let mut rt = Runtime {
        ..Default::default()
    };
    myshell.run(&mut ctx, &mut rt);
}
