use shrs::{
    crossterm::Stylize,
    line::{Prompt, StyledBuf},
    plugin::ShellPlugin,
    ShellConfigBuilder,
};
use shrs_command_timer::CommandTimerPlugin;

struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self) -> StyledBuf {
        StyledBuf::from_iter(vec!["> ".to_string().reset()])
    }
    fn prompt_right(&self) -> StyledBuf {
        StyledBuf::from_iter(vec!["shrs".to_string().reset()])
    }
}

fn main() {
    let myshell = ShellConfigBuilder::default()
        .with_prompt(MyPrompt)
        .with_plugin(CommandTimerPlugin)
        .build()
        .unwrap();
}
