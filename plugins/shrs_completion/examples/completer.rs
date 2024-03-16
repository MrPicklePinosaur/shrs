use shrs::prelude::*;
use shrs_completion::completions::*;

fn main() {
    let mut mycompleter = DefaultCompleter::default();
    mycompleter.register(ssh_rule());

    let myshell = ShellBuilder::default().build().unwrap();

    myshell.run().unwrap();
}
