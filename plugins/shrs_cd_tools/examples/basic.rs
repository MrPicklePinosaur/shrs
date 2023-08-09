use shrs::prelude::*;
use shrs_cd_tools::{rust::CargoToml, DirParsePlugin, DirParseState};

struct MyPrompt;

impl Prompt for MyPrompt {
    fn prompt_left(&self, _line_ctx: &mut LineCtx) -> StyledBuf {
        styled! {
            " > "
        }
    }
    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        // TODO currently very unergonomic
        if let Some(dir_parse_state) = line_ctx.ctx.state.get::<DirParseState>() {
            let rust_info: Option<String> = dir_parse_state
                .get_module_metadata::<CargoToml>("rust")
                .map(|cargo_toml| {
                    format!(
                        "🦀{} {}",
                        cargo_toml.package.edition, cargo_toml.package.name
                    )
                });

            styled! {
                rust_info
            }
        } else {
            styled! {
                "none"
            }
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
