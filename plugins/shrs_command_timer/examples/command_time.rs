use shrs::{
    crossterm::Stylize,
    line::{LineBuilder, LineCtx, Prompt, StyledBuf},
    plugin::ShellPlugin,
    ShellBuilder,
};
use shrs_command_timer::{CommandTimerPlugin, CommandTimerState};

struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self, _line_ctx: &mut LineCtx) -> StyledBuf {
        StyledBuf::from_iter(vec!["> ".to_string().reset()])
    }
    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        let time_str = line_ctx
            .ctx
            .state
            .get::<CommandTimerState>()
            .and_then(|x| x.command_time())
            .map(|x| format!("{x:?}"))
            .unwrap_or(String::new());

        StyledBuf::from_iter(vec![time_str.reset()])
    }
}

fn main() {
    let myline = LineBuilder::default()
        .with_prompt(MyPrompt)
        .build()
        .unwrap();

    let myshell = ShellBuilder::default()
        .with_plugin(CommandTimerPlugin)
        .with_readline(myline)
        .build()
        .unwrap();

    myshell.run();
}
