use shrs::{
    crossterm::Stylize,
    line::{LineBuilder, LineCtx, Prompt, StyledBuf},
    plugin::ShellPlugin,
    ShellConfigBuilder,
};
use shrs_run_context::RunContextPlugin;

fn main() {
    let myline = LineBuilder::default().build().unwrap();

    let myshell = ShellConfigBuilder::default()
        .with_plugin(RunContextPlugin)
        .with_readline(myline)
        .build()
        .unwrap();

    myshell.run();
}
