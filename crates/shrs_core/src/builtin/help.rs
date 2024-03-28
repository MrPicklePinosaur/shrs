use std::fmt::format;

use clap::{Parser, Subcommand};
use crossterm::style::Stylize;
use shrs_utils::styled_buf;

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
    #[arg(global=true)]  // <-- here
    args: Vec<String>
}

#[derive(Subcommand)]
enum Commands {
    Builtin,
    Bindings,
    Plugins,
    Plugin,
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
            Commands::Plugins => {
                ctx.out
                    .println(format!("{} Plugins installed", sh.plugin_metas.len()))?;
                for meta in sh.plugin_metas.iter() {
                    ctx.out.println("")?;

                    ctx.out.print_buf(styled_buf!(meta.name.clone().red()))?;
                    ctx.out.println("")?;
                    ctx.out.println(meta.description.clone())?;
                }
                ctx.out.println("")?;
            },
            Commands::Plugin => {
                let pgn = cli.args.join(" ");
                let mut found = false;
                for meta in sh.plugin_metas.iter() {
                    if meta.name == pgn {
                        found = true;
                        ctx.out.println("")?;
                        ctx.out.print_buf(styled_buf!(meta.name.clone().green()))?;
                        ctx.out.println("")?;
                        ctx.out.println(meta.help.clone())?;
                        break; 
                    }
                }
                if !found {
                    ctx.out.print_buf(styled_buf!("Please specify a valid plugin name: help plugin <plugin-name>").red())?;
                }
                ctx.out.println("")?;
            }
        }

        Ok(CmdOutput::success())
    }
}
