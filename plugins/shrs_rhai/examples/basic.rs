use shrs::prelude::*;
use shrs_rhai::RhaiPlugin;

fn main() {
    let mut env = Env::default();
    env.load();

    let myshell = ShellBuilder::default()
        .with_env(env)
        .with_plugin(RhaiPlugin)
        .build()
        .unwrap();

    myshell.run();
}
