use shrs::prelude::{styled_buf::StyledBuf, *};
use shrs_cd_tools::{
    default_prompt, git::Git, node::NodeJs, rust::CargoToml, DirParsePlugin, DirParseState,
};

struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self, _line_ctx: &LineCtx) -> StyledBuf {
        styled_buf! {
            " > "
        }
    }
    fn prompt_right(&self, line_ctx: &LineCtx) -> StyledBuf {
        // TODO currently very unergonomic
        let project_info = default_prompt(line_ctx);

        let git_branch = line_ctx
            .ctx
            .state
            .get::<DirParseState>()
            .and_then(|state| state.get_module_metadata::<Git>("git"))
            .map(|git| format!("git:{}", git.branch));

        styled_buf! {
            project_info,
            git_branch
        }
    }
}

fn main() {
    let readline = LineBuilder::default()
        .with_prompt(MyPrompt)
        .build()
        .unwrap();

    let myshell = ShellBuilder::default()
        .with_readline(readline)
        .with_plugin(DirParsePlugin::new())
        .build()
        .unwrap();

    myshell.run();
}
