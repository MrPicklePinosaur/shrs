use shrs::{line::LineBuilder, plugin::ShellPlugin, ShellConfigBuilder};
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
