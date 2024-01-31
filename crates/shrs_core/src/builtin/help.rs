use clap::{Parser, Subcommand};

use super::BuiltinCmd;
use crate::{
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

#[derive(Parser)]
#[clap(disable_help_flag = true, disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Builtin,
    Bindings,
}

#[derive(Default)]
pub struct HelpBuiltin {}
impl BuiltinCmd for HelpBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        _rt: &mut Runtime,
        args: &[String],
    ) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        match &cli.command {
            Commands::Builtin => {
                let cmds = sh.builtins.builtins.keys();

                ctx.out.println("Builtin Commands")?;

                for cmd in cmds {
                    ctx.out.println(cmd)?;
                }
            },
            Commands::Bindings => {
                let info = sh.keybinding.get_info();

                ctx.out.println("Key Bindings")?;

                for (binding, desc) in info {
                    ctx.out.println(format!("{}: {}", binding, desc))?;
                }
            },
        }

        Ok(CmdOutput::success())
    }
}
