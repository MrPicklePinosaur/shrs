use rhai::{Engine, EvalAltResult};
use shrs::prelude::*;

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
        let engine = Engine::new();

        for file in args.iter().skip(1) {
            engine.run_file(file.into()).unwrap();
        }

        Ok(CmdOutput::success())
    }
}
