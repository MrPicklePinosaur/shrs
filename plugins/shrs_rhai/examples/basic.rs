use shrs::prelude::*;
use shrs_rhai::RhaiPlugin;

fn main() {
    let mut env = Env::default();
    env.load().expect("unable to load env");

    let myshell = ShellBuilder::default()
        .with_env(env)
        .with_plugin(RhaiPlugin)
        .build()
        .unwrap();

    myshell.run().expect("Error when running shell");
}
