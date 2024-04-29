//! This example shows how to create your own custom builtin and register it
//!
//! The example builtin will be a 'better cd' plugin, which onto of acting like normal cd, also
//! allows you to provide alias specific directories using `cd @work` syntax

use std::{collections::HashMap, path::PathBuf};

use shrs::prelude::*;
use shrs_core::shell::set_working_dir;

struct BetterCdBuiltin {
    aliases: HashMap<String, PathBuf>,
}

impl BetterCdBuiltin {
    pub fn new(aliases: HashMap<String, PathBuf>) -> Self {
        BetterCdBuiltin { aliases }
    }
}

impl Builtin for BetterCdBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &[String],
    ) -> ::anyhow::Result<CmdOutput> {
        // Check if supplied path starts with '@' and is a registered alias
        if let Some(arg) = args.get(0) {
            if let Some(alias) = arg.strip_prefix("@") {
                if let Some(path) = self.aliases.get(alias) {
                    set_working_dir(ctx, path, true).unwrap();
                    return Ok(CmdOutput::success());
                }
            }
        }

        // otherwise just call to default cd builtin
        let _ = CdBuiltin::default().run(ctx, args);

        Ok(CmdOutput::success())
    }
}

fn main() {
    // use Builtins::default() since it gives us some default builtins to use, rather than
    // Builtins::new() which gives us nothing
    let mut builtins = Builtins::default();

    let aliases = HashMap::from_iter([
        ("work".into(), PathBuf::from("~/Documents/work")),
        (
            "homework".into(),
            PathBuf::from("~/School/undergrad/4a/cs452"),
        ),
    ]);

    // register our custom builtin to override the default cd
    let better_cd = BetterCdBuiltin::new(aliases);
    builtins.insert("cd", better_cd);

    let myshell = ShellBuilder::default()
        .with_builtins(builtins)
        .build()
        .unwrap();

    myshell.run().unwrap();
}
