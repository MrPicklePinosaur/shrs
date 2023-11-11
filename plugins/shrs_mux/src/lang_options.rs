use std::collections::HashMap;

use shrs::{
    anyhow,
    prelude::{Context, Highlighter, Runtime, Shell},
};

use crate::ChangeLangCtx;

pub struct LangOptions {
    highlighters: HashMap<String, Box<dyn Highlighter>>,
}
impl LangOptions {
    pub fn new(highlighters: HashMap<String, Box<dyn Highlighter>>) -> Self {
        LangOptions { highlighters }
    }
}
impl Default for LangOptions {
    fn default() -> Self {
        let highlighters: HashMap<String, Box<dyn Highlighter>> = HashMap::from([]);
        Self { highlighters }
    }
}
pub(crate) fn swap_lang_options(
    sh: &Shell,
    sh_ctx: &mut Context,
    sh_rt: &mut Runtime,
    ctx: &ChangeLangCtx,
) -> anyhow::Result<()> {
    Ok(())
}
