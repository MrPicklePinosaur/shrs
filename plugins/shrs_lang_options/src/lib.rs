use std::{collections::HashMap, default, usize};

use shrs::prelude::*;
use shrs_mux::ChangeLangCtx;

pub struct LangOptionsPlugin {
    highlighters: HashMap<String, Box<dyn Highlighter>>,
}
impl LangOptionsPlugin {
    pub fn new(highlighters: HashMap<String, Box<dyn Highlighter>>) -> Self {
        LangOptionsPlugin { highlighters }
    }
}
impl Default for LangOptionsPlugin {
    fn default() -> Self {
        let highlighters: HashMap<String, Box<dyn Highlighter>> = HashMap::from([]);
        Self { highlighters }
    }
}

impl Plugin for LangOptionsPlugin {
    fn init(&self, shell: &mut shrs::ShellConfig) -> anyhow::Result<()> {
        shell.hooks.register(swap_lang_options);

        Ok(())
    }
}
fn swap_lang_options(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &ChangeLangCtx,
) -> anyhow::Result<()> {
    Ok(())
}
