use shrs::prelude::*;

// TODO make shell mode part of state so we can modify from anywhere?
// TODO add custom hook from when we switch shell mode

#[derive(Default)]
pub struct MuxBuiltin {}

impl MuxBuiltin {
    pub fn new() -> Self {
        MuxBuiltin {}
    }
}

impl BuiltinCmd for MuxBuiltin {
    fn run(
        &self,
        _sh: &Shell,
        ctx: &mut Context,
        _rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<std::process::Child> {
        // TODO hardcoded for now
        // TODO think about how to implement shell switching at runtime (currently running into
        // some ownership issues in shrs/shell.rs)
        match args.get(0).map(|s| s.as_str()) {
            Some("nu") => {},
            Some("shrs") => {},
            _ => return dummy_child(),
        };

        dummy_child()
    }
}
