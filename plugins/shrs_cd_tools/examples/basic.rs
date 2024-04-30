use shrs::prelude::{styled_buf::StyledBuf, *};
use shrs_cd_tools::{default_prompt, git::Git, DirParsePlugin, DirParseState};

fn prompt_left(sh: &Shell) -> StyledBuf {
    styled_buf! {
        " > "
    }
}
fn prompt_right(state: State<DirParseState>, sh: &Shell) -> StyledBuf {
    let project_info = default_prompt(&state, sh);

    let git_branch = state
        .get_module_metadata::<Git>("git")
        .map(|git| format!("git:{}", git.branch));

    styled_buf! {
        project_info,
        git_branch
    }
}

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(DirParsePlugin::new())
        .with_prompt(Prompt::from_sides(prompt_left, prompt_right))
        .build()
        .unwrap();

    myshell.run().expect("Error while running shell");
}
