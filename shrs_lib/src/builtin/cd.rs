use std::{env, path::Path};

use super::BuiltinCmd;
use crate::shell::{dummy_child, Context, Runtime};

#[derive(Default)]
pub struct CdBuiltin {}

impl BuiltinCmd for CdBuiltin {
    fn run(
        &self,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<std::process::Child> {
        // if empty default to root (for now)
        let raw_path = if let Some(path) = args.get(0) {
            path
        } else {
            "/"
        };

        let path = Path::new(raw_path);
        let new_path = rt.working_dir.join(path);
        // env::set_current_dir(path)?; // env current dir should just remain as the directory the shell was started in
        rt.working_dir = new_path.clone();
        rt.env.set("PWD", new_path.to_str().unwrap());

        // return a dummy command
        dummy_child()
    }
}
