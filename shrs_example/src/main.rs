#[macro_use]
extern crate shrs;

use std::{
    fs,
    io::{stdout, BufWriter},
    process::Command,
};

use shrs::{line::_core::shell::set_working_dir, prelude::*};
use shrs_cd_stack::{CdStackPlugin, CdStackState};
use shrs_cd_tools::git;
use shrs_command_timer::{CommandTimerPlugin, CommandTimerState};
use shrs_mux::{MuxPlugin, MuxState};
use shrs_output_capture::OutputCapturePlugin;
use shrs_run_context::RunContextPlugin;

// =-=-= Prompt customization =-=-=
// Create a new struct and implement the [Prompt] trait
struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        let indicator = match line_ctx.mode() {
            LineMode::Insert => String::from(">").cyan(),
            LineMode::Normal => String::from(":").yellow(),
        };
        if !line_ctx.lines.is_empty() {
            return styled! {" ", indicator, " "};
        }

        styled! {" ", @(blue)username(), " ", @(white,bold)top_pwd(), " ", indicator, " "}
    }
    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        let time_str = line_ctx
            .ctx
            .state
            .get::<CommandTimerState>()
            .and_then(|x| x.command_time())
            .map(|x| format!("{x:?}"));

        let lang = line_ctx
            .ctx
            .state
            .get::<MuxState>()
            .map(|state| state.get_lang());

        let git_branch = git::branch().map(|s| format!("git:{s}"));
        if !line_ctx.lines.is_empty() {
            return styled! {""};
        }

        styled! {@(bold,blue)git_branch, " ", time_str, " ", lang, " "}
    }
}

fn main() {
    env_logger::init();

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

    let builtins = Builtins::default();

    // =-=-= Completion =-=-=
    // Get list of binaries in path and initialize the completer to autocomplete command names
    let path_string = env.get("PATH").unwrap().to_string();
    let mut completer = DefaultCompleter::default();
    completer.register(Rule::new(
        Pred::new(cmdname_pred),
        Box::new(cmdname_action(path_string)),
    ));
    completer.register(Rule::new(
        Pred::new(cmdname_pred),
        Box::new(builtin_cmdname_action(&builtins)),
    ));

    // =-=-= Menu =-=-=-=
    let menu = DefaultMenu::new();

    // =-=-= History =-=-=
    // Use history that writes to file on disk
    let history_file = config_dir.as_path().join("history");
    let history = FileBackedHistory::new(history_file).expect("Could not open history file");

    let highlighter = SyntaxHighlighter::new(SyntaxTheme::default());

    // =-=-= Keybindings =-=-=
    // Add basic keybindings
    let keybinding = keybindings! {
        |sh, ctx, rt|
        "C-l" => { Command::new("clear").spawn() },
        "C-p" => {
            if let Some(state) = ctx.state.get_mut::<CdStackState>() {
                if let Some(new_path) = state.pop() {
                    set_working_dir(sh, ctx, rt, &new_path).unwrap();
                    // TODO hacky, removing the directory we just went to
                    // state.pop();
                }
            }
        },
    };

    // =-=-= Prompt =-=-=
    let prompt = MyPrompt;

    // =-=-= Readline =-=-=
    // Initialize readline with all of our components

    let readline = LineBuilder::default()
        .with_completer(completer)
        .with_menu(menu)
        .with_history(history)
        .with_highlighter(highlighter)
        .with_keybinding(keybinding)
        .with_prompt(prompt)
        .build()
        .expect("Could not construct readline");

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
    let mut hooks = Hooks::new();
    hooks.register(startup_msg);

    // =-=-= Shell =-=-=
    // Construct the final shell
    let myshell = ShellBuilder::default()
        .with_hooks(hooks)
        .with_env(env)
        .with_alias(alias)
        .with_readline(readline)
        .with_plugin(OutputCapturePlugin)
        .with_plugin(CommandTimerPlugin)
        .with_plugin(RunContextPlugin)
        .with_plugin(MuxPlugin)
        .with_plugin(CdStackPlugin)
        .build()
        .expect("Could not construct shell");

    myshell.run();
}
