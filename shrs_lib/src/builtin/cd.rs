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
        let path = if let Some(path) = args.get(0) {
            rt.working_dir.join(Path::new(path))
        } else {
            let home_dir = rt.env.get("HOME").unwrap();
            Path::new(home_dir).to_path_buf()
        };

        env::set_current_dir(path.clone())?; // TODO should env current dir remain as the directory the shell was started in?
        rt.working_dir = path.clone();
        rt.env.set("PWD", path.to_str().unwrap());

        // return a dummy command
        dummy_child()
    }
}
