use std::{
    default,
    io::{stdout, BufWriter},
};

use anyhow::Result;
use crossterm::style::Stylize;
use shrs::{
    builtin::Builtins,
    hooks::{HookFn, HookList, Hooks, StartupCtx},
    line::{
        completion::{
            new_filepath_completer, BetterCompleter, CompletionCtx, DefaultCompleter, Rule,
        },
        DefaultCursor, DefaultHighlighter, DefaultHistory, DefaultMenu, Line, LineBuilder, Prompt,
    },
    prompt::{hostname, top_pwd, username},
    Alias, Context, Env, Runtime, Shell, ShellConfig, ShellConfigBuilder,
};
use shrs_output_capture::OutputCapturePlugin;

struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self) -> String {
        // let path = top_pwd().white().bold();
        // let username = username().unwrap_or_default().blue();
        // let hostname = hostname().unwrap_or_default().blue();
        // let prompt = ">".blue();
        let path = top_pwd();
        let username = username().unwrap_or_default();
        let hostname = hostname().unwrap_or_default();
        let prompt = ">";
        format!("{hostname}@{username} {path} {prompt} ")
    }
    fn prompt_right(&self) -> String {
        format!("shrs ")
    }
}

fn main() {
    let mut out = BufWriter::new(stdout());

    let mut env = Env::new();
    env.load();

    // configure line
    let completer = BetterCompleter::default();

    let menu = DefaultMenu::new();
    let history = DefaultHistory::new();
    let cursor = DefaultCursor::default();
    let highlighter = DefaultHighlighter::default();

    let readline = LineBuilder::default()
        .with_cursor(cursor)
        .with_completer(completer)
        .with_menu(menu)
        .with_history(history)
        .with_highlighter(highlighter)
        .build()
        .unwrap();

    let prompt = MyPrompt;

    let alias = Alias::from_iter([
        ("l".into(), "ls".into()),
        ("c".into(), "cd".into()),
        ("g".into(), "git".into()),
        ("v".into(), "vim".into()),
        ("la".into(), "ls -a".into()),
    ]);

    let startup_msg: HookFn<StartupCtx> = |sh: &Shell,
                                           sh_ctx: &mut Context,
                                           sh_rt: &mut Runtime,
                                           _ctx: &StartupCtx|
     -> anyhow::Result<()> {
        let welcome_str = format!(
            r#"
        __         
   ___ / /  _______
  (_-</ _ \/ __(_-<
 /___/_//_/_/ /___/
a rusty POSIX shell | build {}"#,
            env!("SHRS_VERSION")
        );

        println!("{}", welcome_str);
        Ok(())
    };

    let hooks = Hooks {
        startup: HookList::from_iter(vec![startup_msg]),
        ..Default::default()
    };

    let myshell = ShellConfigBuilder::default()
        .with_hooks(hooks)
        .with_env(env)
        .with_alias(alias)
        .with_readline(readline)
        .with_prompt(prompt)
        .with_plugin(OutputCapturePlugin)
        .build()
        .unwrap();

    myshell.run();
}
