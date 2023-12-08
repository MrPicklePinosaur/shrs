use shrs::prelude::*;
use shrs_completion::completions::*;

fn main() {
    let mut mycompleter = DefaultCompleter::default();
    ssh_completion(&mut mycompleter);

    let myline = LineBuilder::default()
        .with_completer(mycompleter)
        .build()
        .unwrap();

    let myshell = ShellBuilder::default()
        .with_readline(myline)
        .build()
        .unwrap();

    myshell.run().unwrap();
}
