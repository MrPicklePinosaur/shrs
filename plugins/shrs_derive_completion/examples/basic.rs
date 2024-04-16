use shrs::prelude::*;
use shrs_derive_completion::Completion;

#[derive(Completion)]
#[allow(unused)]
struct MyCli {
    #[flag(long = "help", short)]
    help: bool,
    #[flag(short = "v")]
    verbose: bool,
}

fn main() {
    let mut comp = DefaultCompleter::new();
    MyCli::rules(&mut comp);

    let myshell = ShellBuilder::default()
        .with_completer(comp)
        .build()
        .unwrap();

    myshell.run().expect("Error while running shell");
}
