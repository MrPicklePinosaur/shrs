use shrs::prelude::*;
use shrs_cd_stack::{CdStackPlugin, CdStackState};

fn cd_stack_down(
    mut rt: StateMut<Runtime>,
    mut state: StateMut<CdStackState>,
    sh: &Shell,
) -> anyhow::Result<()> {
    if let Some(path) = state.down() {
        let _ = set_working_dir(sh, &mut rt, &path, true);
    }
    Ok(())
}

fn cd_stack_up(
    mut rt: StateMut<Runtime>,
    mut state: StateMut<CdStackState>,
    sh: &Shell,
) -> anyhow::Result<()> {
    if let Some(path) = state.up() {
        let _ = set_working_dir(sh, &mut rt, &path, true);
    }
    Ok(())
}

fn main() {
    let mut bindings = Keybindings::new();
    bindings
        .insert("C-p", "Go back in path history", cd_stack_down)
        .unwrap();
    bindings
        .insert("C-n", "Go back in path history", cd_stack_up)
        .unwrap();

    let myshell = ShellBuilder::default()
        .with_plugin(CdStackPlugin)
        .with_keybinding(bindings)
        .build()
        .unwrap();

    myshell.run().expect("Error while running shell");
}
