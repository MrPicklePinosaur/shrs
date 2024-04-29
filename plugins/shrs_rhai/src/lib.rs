mod builtin;
pub mod rhai;

use builtin::command_not_found_hook;
use rhai::{RhaiEngine, RhaiScope, RhaiAST};
use shrs::prelude::*;

pub struct RhaiPlugin;

impl Plugin for RhaiPlugin {
    fn init(&self, config: &mut ShellConfig) -> anyhow::Result<()> {
        config.builtins.insert("source", builtin::rhai_builtin);
        config.hooks.insert(command_not_found_hook);
        config.states.insert(RhaiEngine::new());
        config.states.insert(RhaiScope::new());
        config.states.insert(RhaiAST::new());
        Ok(())
    }

    // TODO we need to also pass in Shell here?
    fn post_init(&self, sh: &mut Shell, states: &mut States) -> anyhow::Result<()> {
        // TODO path currently not configurable
        // source `init.rhai` if exists

        if let Some(mut home_dir) = dirs::home_dir() {
            home_dir.push(".config/shrs/init.rhai");
            // TODO maybe warn if not sourced?
            let _ = sh.builtins.get("source").unwrap().run(
                sh,
                states,
                &vec!["source".to_string(), home_dir.to_str().unwrap().to_string()],
            );
        }

        Ok(())
    }
}
