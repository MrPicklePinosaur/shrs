use std::{collections::HashMap, process::Command};

use shrs::prelude::*;

use crate::MuxState;

pub struct MuxLang {
    langs: HashMap<String, Box<dyn Lang>>,
}

impl MuxLang {
    pub fn new() -> Self {
        // TODO should be configurable later
        Self {
            langs: HashMap::from_iter(vec![
                (
                    "shrs".into(),
                    Box::new(PosixLang::default()) as Box<dyn Lang>,
                ),
                ("nu".into(), Box::new(NuLang::new()) as Box<dyn Lang>),
            ]),
        }
    }
}

impl Lang for MuxLang {
    fn eval(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        cmd: String,
    ) -> anyhow::Result<()> {
        let lang_name = match ctx.state.get::<MuxState>() {
            Some(state) => &state.lang,
            None => return Ok(()),
        };
        // TODO maybe return error if we can't find a lang

        self.langs.get(lang_name).map(|lang| {
            lang.eval(sh, ctx, rt, cmd);
        });

        Ok(())
    }
}

pub struct NuLang {}

impl NuLang {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lang for NuLang {
    fn eval(
        &self,
        sh: &shrs::Shell,
        ctx: &mut shrs::Context,
        rt: &mut shrs::Runtime,
        cmd: String,
    ) -> shrs::anyhow::Result<()> {
        let mut words_it = cmd
            .split(' ')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        // Retrieve command name or return immediately (empty command)
        let cmd_name = match words_it.next() {
            Some(cmd_name) => cmd_name,
            None => return Ok(()),
        };
        let args = words_it
            .map(|s| s.to_owned().to_string())
            .collect::<Vec<_>>();

        for (builtin_name, builtin_cmd) in sh.builtins.iter() {
            if builtin_name == &cmd_name {
                builtin_cmd.run(sh, ctx, rt, &args)?;
                return Ok(());
            }
        }

        let mut handle = Command::new("nu").args(vec!["-c", &cmd]).spawn()?;

        handle.wait()?;

        Ok(())
    }
}
