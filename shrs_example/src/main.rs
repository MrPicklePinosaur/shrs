use std::{
    default, fs,
    io::{stdout, BufWriter},
    path::Path,
    process::Command,
};

use anyhow::Result;
use crossterm::{
    event::{KeyCode, KeyModifiers},
    style::Stylize,
};
use shrs::{
    builtin::Builtins,
    hooks::{HookFn, HookList, Hooks, StartupCtx},
    line::{
        completion::{cmdname_action, cmdname_pred, CompletionCtx, DefaultCompleter, Pred, Rule},
        DefaultCursor, DefaultHighlighter, DefaultHistory, DefaultKeybinding, DefaultMenu,
        FileBackedHistory, Keybinding, Line, LineBuilder, Prompt, StyledBuf,
    },
    prompt::{hostname, top_pwd, username},
    Alias, Context, Env, Runtime, Shell, ShellConfig, ShellConfigBuilder,
};
use shrs_output_capture::OutputCapturePlugin;

struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self) -> StyledBuf {
        StyledBuf::from_iter(vec![
            username().unwrap_or_default().blue(),
            String::from("@").reset(),
            hostname().unwrap_or_default().blue(),
            String::from(" ").reset(),
            top_pwd().white().bold(),
            String::from(" ").reset(),
            "> ".to_string().blue(),
        ])
    }
    fn prompt_right(&self) -> StyledBuf {
        StyledBuf::from_iter(vec!["shrs".to_string().blue(), String::from(" ").reset()])
    }
}

fn main() {
    let mut out = BufWriter::new(stdout());

    let mut env = Env::new();
    env.load();

    // configure line
    let path_string = env.get("PATH").unwrap().to_string();
    let mut completer = DefaultCompleter::default();
    completer.register(Rule::new(
        Pred::new(cmdname_pred),
        Box::new(cmdname_action(path_string)),
    ));

    let menu = DefaultMenu::new();

    // init config directory
    let config_dir = dirs::home_dir().unwrap().as_path().join(".config/shrs");
    // also log when creating dir
    // TODO ignore errors for now (we dont care if dir already exists)
    fs::create_dir_all(config_dir.clone());

    let history_file = config_dir.as_path().join("history");
    let history = FileBackedHistory::new(history_file).unwrap();

    let cursor = DefaultCursor::default();
    let highlighter = DefaultHighlighter::default();
    let keybinding = DefaultKeybinding::from_iter([(
        (KeyCode::Char('l'), KeyModifiers::CONTROL),
        Box::new(|| {
            Command::new("clear").spawn();
        }) as Box<dyn FnMut()>,
    )]);

    let readline = LineBuilder::default()
        .with_cursor(cursor)
        .with_completer(completer)
        .with_menu(menu)
        .with_history(history)
        .with_highlighter(highlighter)
        .with_keybinding(keybinding)
        .build()
        .unwrap();

    let prompt = MyPrompt;

    let alias = Alias::from_iter([
        ("ls".into(), "ls --color=auto".into()),
        ("l".into(), "ls --color=auto".into()),
        ("c".into(), "cd".into()),
        ("g".into(), "git".into()),
        ("v".into(), "vim".into()),
        ("V".into(), "nvim".into()),
        ("la".into(), "ls -a --color=auto".into()),
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
