use shrs::{line::LineBuilder, ShellConfigBuilder};
use shrs_derive_completion::Completion;

#[derive(Completion)]
struct MyCli {
    #[flag(long = "help", short)]
    help: bool,
    #[flag(short = "v")]
    verbose: bool,
}

fn main() {
    let mut comp = DefaultCompleter::new();
    MyCli::rules(&mut comp);

    let readline = LineBuilder::default().with_completer(comp).build().unwrap();

    let myshell = ShellConfigBuilder::default()
        .with_readline(readline)
        .build()
        .unwrap();

    myshell.run();
}
