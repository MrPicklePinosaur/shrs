use shrs::{line::LineBuilder, ShellConfigBuilder};
use shrs_derive_completion::Completion;

#[derive(Completion)]
struct MyCompletion {
    #[flag(long = "help")]
    help: bool,
}

fn main() {
    let mut comp = DefaultCompleter::default();
    MyCompletion::rules(&mut comp);

    let readline = LineBuilder::default().with_completer(comp).build().unwrap();

    let myshell = ShellConfigBuilder::default().build().unwrap();
}
