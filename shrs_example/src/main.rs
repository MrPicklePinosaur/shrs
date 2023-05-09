use std::{
    fs,
    io::{stdout, BufWriter},
    process::Command,
};

use crossterm::{
    event::{KeyCode, KeyModifiers},
    style::Stylize,
};
use shrs::{
    hooks::{HookFn, HookList, Hooks, StartupCtx},
    line::{
        completion::{cmdname_action, cmdname_pred, DefaultCompleter, Pred, Rule},
        DefaultCursor, DefaultHighlighter, DefaultKeybinding, DefaultMenu, FileBackedHistory,
        LineBuilder, Prompt, StyledBuf,
    },
    prompt::{hostname, top_pwd, username},
    Alias, Context, Env, Runtime, Shell, ShellConfigBuilder,
};
use shrs_output_capture::OutputCapturePlugin;

// =-=-= Prompt customization =-=-=
// Create a new struct and implement the [Prompt] trait
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
    let _out = BufWriter::new(stdout());

    // =-=-= Configuration directory =-=-=
    // Initialize the directory we will be using to hold our configuration and metadata files
    let config_dir = dirs::home_dir().unwrap().as_path().join(".config/shrs");
    // also log when creating dir
    // TODO ignore errors for now (we dont care if dir already exists)
    fs::create_dir_all(config_dir.clone());

    // =-=-= Environment variables =-=-=
    // Load environment variables from calling shell
    let mut env = Env::new();
    env.load();
    env.set("SHELL_NAME", "shrs_example");

    // =-=-= Completion =-=-=
    // Get list of binaries in path and initialize the completer to autocomplete command names
    let path_string = env.get("PATH").unwrap().to_string();
    let mut completer = DefaultCompleter::default();
    completer.register(Rule::new(
        Pred::new(cmdname_pred),
        Box::new(cmdname_action(path_string)),
    ));

    // =-=-= Menu =-=-=-=
    let menu = DefaultMenu::new();

    // =-=-= History =-=-=
    // Use history that writes to file on disk
    let history_file = config_dir.as_path().join("history");
    let history = FileBackedHistory::new(history_file).unwrap();

    let cursor = DefaultCursor::default();
    let highlighter = DefaultHighlighter::default();

    // =-=-= Keybindings =-=-=
    // Add basic keybindings
    let keybinding = DefaultKeybinding::from_iter([(
        (KeyCode::Char('l'), KeyModifiers::CONTROL),
        Box::new(|| {
            Command::new("clear").spawn();
        }) as Box<dyn FnMut()>,
    )]);

    // =-=-= Prompt =-=-=
    let prompt = MyPrompt;

    // =-=-= Readline =-=-=
    // Initialize readline with all of our components

    let readline = LineBuilder::default()
        .with_cursor(cursor)
        .with_completer(completer)
        .with_menu(menu)
        .with_history(history)
        .with_highlighter(highlighter)
        .with_keybinding(keybinding)
        .with_prompt(prompt)
        .build()
        .unwrap();

    // =-=-= Aliases =-=-=
    // Set aliases
    let alias = Alias::from_iter([
        ("ls", "ls --color=auto"),
        ("l", "ls --color=auto"),
        ("c", "cd"),
        ("g", "git"),
        ("v", "vim"),
        ("V", "nvim"),
        ("la", "ls -a --color=auto"),
    ]);

    // =-=-= Hooks =-=-=
    // Create a hook that prints a welcome message on startup
    let startup_msg: HookFn<StartupCtx> = |_sh: &Shell,
                                           _sh_ctx: &mut Context,
                                           _sh_rt: &mut Runtime,
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

        println!("{welcome_str}");
        Ok(())
    };
    let hooks = Hooks {
        startup: HookList::from_iter(vec![startup_msg]),
        ..Default::default()
    };

    // =-=-= Shell =-=-=
    // Construct the final shell
    let myshell = ShellConfigBuilder::default()
        .with_hooks(hooks)
        .with_env(env)
        .with_alias(alias)
        .with_readline(readline)
        .with_plugin(OutputCapturePlugin)
        .build()
        .unwrap();

    myshell.run();
}
