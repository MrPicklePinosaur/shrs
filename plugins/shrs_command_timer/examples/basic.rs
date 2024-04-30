use shrs::prelude::{styled_buf::StyledBuf, *};
use shrs_command_timer::{CommandTimerPlugin, CommandTimerState};

fn prompt_left(shell: &Shell) -> StyledBuf {
    styled_buf!("> ")
}

fn prompt_right(state: State<CommandTimerState>, shell: &Shell) -> StyledBuf {
    let time_str = state
        .command_time()
        .map(|x| format!("{x:?}"))
        .unwrap_or(String::new());
    styled_buf!(time_str.reset())
}

fn main() {
    let prompt = Prompt::from_sides(prompt_left, prompt_right);

    let myshell = ShellBuilder::default()
        .with_plugin(CommandTimerPlugin)
        .with_prompt(prompt)
        .build()
        .unwrap();

    myshell.run().expect("Shell Failed");
}
