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
        let package_name: Option<String> = line_ctx
            .ctx
            .state
            .get::<DirParseState>()
            .and_then(|state| state.get_module("rust"))
            .and_then(|rust_mod| rust_mod.get_metadata::<CargoToml>())
            .map(|cargo_toml| {
                format!(
                    "🦀{} {}",
                    cargo_toml.package.edition, cargo_toml.package.name
                )
            });

        styled! {
            package_name
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
