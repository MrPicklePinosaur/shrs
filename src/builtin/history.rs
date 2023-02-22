//! The shell builtin that wraps functionality of the History module

// debatable if crate::history should be moved to crate::builtin::history

use std::{
    io::{stdout, Write},
    process::Child,
};

use pino_argparse::{Cli, Command, FlagParse};

use crate::shell::{dummy_child, Context, Shell};

pub fn history_builtin(ctx: &mut Context, args: &Vec<String>) -> anyhow::Result<Child> {
    let cli = Cli {
        program_name: "history",
        synopsis: "query and modify shell history",
        root_command: Command {
            flags: vec![],
            handler: move |flagparse: FlagParse| -> Result<(), Box<dyn std::error::Error>> {
                // let history = ctx.history.all();
                // for (i, h) in history.iter().enumerate() {
                //     print!("{} {}", i, h);
                // }
                // stdout().flush()?;
                Ok(())
            },
            ..Default::default()
        },
        ..Default::default()
    };

    cli.run(args);
    dummy_child()
}
