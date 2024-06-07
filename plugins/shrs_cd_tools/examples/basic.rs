use shrs::prelude::{StyledBuf, *};
use shrs_cd_tools::{
    default_prompt,
    git::{self, Git, GitPlugin},
    DirParsePlugin, DirParseState,
};

fn prompt_left() -> StyledBuf {
    styled_buf! {
        " > "
    }
}
fn prompt_right(mut git: StateMut<Git>, sh: &Shell) -> StyledBuf {
    if git.is_repo() {
        return styled_buf!(git.stashes().unwrap().to_string());
    }
    return styled_buf!();
}

fn main() {
    let myshell = ShellBuilder::default()
        .with_plugin(DirParsePlugin::new())
        .with_plugin(GitPlugin)
        .with_prompt(Prompt::from_sides(prompt_left, prompt_right))
        .build()
        .unwrap();

    myshell.run().expect("Error while running shell");
}
