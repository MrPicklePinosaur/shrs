use shrs::prelude::*;
use std::{collections::HashMap, default, usize};

pub struct LangOptionsPlugin {
    highlighters: HashMap<String, Box<dyn Highlighter>>,
    need_line_checks: HashMap<String, LineCheckFn>,
}

impl Plugin for LangOptionsPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) {
        shell.hooks.register(swap_lang_options);
        // shell.state.insert(InsulterState::new(
        //     self.insults.clone(),
        //     self.freq,
        //     self.include_default,
        // ));
    }
}
fn swap_lang_options(
    _sh: &Shell,
    sh_ctx: &mut Context,
    _sh_rt: &mut Runtime,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    Ok(())
}
