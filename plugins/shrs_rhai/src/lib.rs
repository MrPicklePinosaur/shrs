mod builtin;
mod rhai;

use builtin::command_not_found_hook;
use rhai::RhaiState;
use shrs::prelude::*;

pub struct RhaiPlugin;

impl Plugin for RhaiPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.builtins.insert("source", builtin::RhaiBuiltin::new());
        shell.hooks.insert(command_not_found_hook);
        shell.state.insert(RhaiState::new());
        Ok(())
    }

    fn post_init(&self, sh: &Shell, ctx: &mut Context, rt: &mut Runtime) -> anyhow::Result<()> {
        // TODO path currently not configurable
        // source `init.rhai` if exists
        if let Some(mut home_dir) = dirs::home_dir() {
            home_dir.push(".config/shrs/init.rhai");
            // TODO maybe warn if not sourced?
            let _ = sh.builtins.get("source").unwrap().run(
                sh,
                ctx,
                rt,
                &vec!["source".to_string(), home_dir.to_str().unwrap().to_string()],
            );
        }

        Ok(())
    }
}
