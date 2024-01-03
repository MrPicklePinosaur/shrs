use rhai::{Engine, EvalAltResult};
use shrs::prelude::*;

use crate::rhai::create_engine;

#[derive(Default)]
pub struct RhaiBuiltin {}

impl RhaiBuiltin {
    pub fn new() -> Self {
        Self {}
    }
}

impl BuiltinCmd for RhaiBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let engine = create_engine(sh, ctx, rt);

        for file in args.iter().skip(1) {
            let _ = engine.run_file(file.into()).map_err(|e| eprintln!("{}", e));
        }

        Ok(CmdOutput::success())
    }
}
