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
}

#[derive(Subcommand)]
enum Commands {
    Builtin,
    Bindings,
    Plugin {
        plugin_name: Vec<String>
    },
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
            Commands::Plugin{plugin_name} => {
                let plg_name = plugin_name.join(" ");
                if plg_name.len() == 0 {
                    ctx.out
                        .println(format!("{} Plugins installed", sh.plugin_metas.len()))?;
                    for meta in sh.plugin_metas.iter() {
                        ctx.out.println("")?;

                        ctx.out.print_buf(styled_buf!(meta.name.clone().red()))?;
                        ctx.out.println("")?;
                        ctx.out.println(meta.description.clone())?;
                    }
                }else {
                    let mut found = false;
                    for meta in sh.plugin_metas.iter() {
                        if meta.name == plg_name {
                            found = true;
                            ctx.out.println("")?;
                            ctx.out.print_buf(styled_buf!(meta.name.clone().green()))?;
                            ctx.out.println("")?;
                            match &meta.help {
                                Some(help_text) => ctx.out.println(help_text.clone())?,
                                None => ctx.out.println("No help message provided.")?,
                            }
                            break; 
                        }
                    }
                    if !found {
                        ctx.out.println("")?;
                        ctx.out.print_buf(styled_buf!(format!("'{}' was not found, please specify a valid plugin name.", plg_name)).red())?;
                        ctx.out.println("")?;
                    }
                }
                ctx.out.println("")?;
            }
        }

        Ok(CmdOutput::success())
    }
}
