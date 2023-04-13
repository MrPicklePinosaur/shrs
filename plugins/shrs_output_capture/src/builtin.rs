use std::{
    env,
    path::{Path, PathBuf},
};

use shrs::{anyhow, builtin::BuiltinCmd, dummy_child, Context, Runtime};

#[derive(Default)]
pub struct AgainBuiltin {
    last_output: String,
}

impl AgainBuiltin {
    pub fn new() -> Self {
        AgainBuiltin {
            last_output: String::new(),
        }
    }

    pub fn update(&mut self, output: String) {
        self.last_output = output;
    }

    pub fn get(&self) -> &String {
        &self.last_output
    }
}

impl BuiltinCmd for AgainBuiltin {
    fn run(
        &self,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<std::process::Child> {
        println!("{}", self.get());

        dummy_child()
    }
}
