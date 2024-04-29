use clap::{Parser, Subcommand};
use crossterm::style::Stylize;
use shrs_utils::styled_buf;

use super::Builtin;
use crate::{
    prelude::{CmdOutput, OutputWriter, Shell, State, States},
    shell::PluginMetas,
    state::StateMut,
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
    Plugin { plugin_name: Vec<String> },
}

pub fn help_builtin(
    mut out: StateMut<OutputWriter>,
    plugin_metas: State<PluginMetas>,
    sh: &Shell,
    args: &Vec<String>,
) -> anyhow::Result<CmdOutput> {
    let cli = Cli::try_parse_from(args)?;

    match &cli.command {
        Commands::Builtin => {
            let cmds = sh.builtins.builtins.keys();

            out.println("Builtin Commands")?;

            for cmd in cmds {
                out.println(cmd)?;
            }
        },
        Commands::Bindings => {
            let info = sh.keybinding.get_info();

            out.println("Key Bindings")?;

            for (binding, desc) in info {
                out.println(format!("{}: {}", binding, desc))?;
            }
        },
        Commands::Plugin { plugin_name } => {
            let plugin_name = plugin_name.join(" ");
            if plugin_name.len() == 0 {
                out.println(format!("{} Plugins installed", plugin_metas.len()))?;
                for meta in plugin_metas.iter() {
                    out.println("")?;

                    out.print_buf(styled_buf!(meta.name.clone().red()))?;
                    out.println("")?;
                    out.println(meta.description.clone())?;
                }
            } else {
                let mut found = false;
                for meta in plugin_metas.iter() {
                    if meta.name == plugin_name {
                        found = true;
                        out.println("")?;
                        out.print_buf(styled_buf!(meta.name.clone().green()))?;
                        out.println("")?;
                        match &meta.help {
                            Some(help_text) => out.println(help_text.clone())?,
                            None => out.println("No help message provided.")?,
                        }
                        break;
                    }
                }
                if !found {
                    out.println("")?;
                    out.print_buf(
                        styled_buf!(format!(
                            "'{}' was not found, please specify a valid plugin name.",
                            plugin_name
                        ))
                        .red(),
                    )?;
                    out.println("")?;
                }
            }
            out.println("")?;
        },
    }

    Ok(CmdOutput::success())
}
