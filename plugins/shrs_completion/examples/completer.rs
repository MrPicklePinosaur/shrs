use shrs::prelude::*;
use shrs_completion::completions::*;

fn main() {
    let mut mycompleter = Completer::default();
    ssh_completion(&mut mycompleter);

    let myshell = ShellBuilder::default().build().unwrap();

    myshell.run().unwrap();
}
