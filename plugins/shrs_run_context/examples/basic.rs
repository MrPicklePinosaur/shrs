use shrs::{line::LineBuilder, plugin::ShellPlugin, ShellBuilder};
use shrs_run_context::RunContextPlugin;

fn main() {
    let myline = LineBuilder::default().build().unwrap();

    let myshell = ShellBuilder::default()
        .with_plugin(RunContextPlugin)
        .with_readline(myline)
        .build()
        .unwrap();

    myshell.run();
}
