use clap::{Parser, Subcommand};
use crossterm::style::Stylize;
use shrs_utils::styled_buf;

use super::BuiltinCmd;
use crate::{
    prelude::{CmdOutput, OutputWriter, Shell, States},
    shell::PluginMetas,
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

#[derive(Default)]
pub struct HelpBuiltin {}
impl BuiltinCmd for HelpBuiltin {
    fn run(&self, sh: &Shell, states: &mut States, args: &[String]) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        match &cli.command {
            Commands::Builtin => {
                let cmds = sh.builtins.builtins.keys();

                states
                    .get_mut::<OutputWriter>()
                    .println("Builtin Commands")?;

                for cmd in cmds {
                    states.get_mut::<OutputWriter>().println(cmd)?;
                }
            },
            Commands::Bindings => {
                let info = sh.keybinding.get_info();

                states.get_mut::<OutputWriter>().println("Key Bindings")?;

                for (binding, desc) in info {
                    states
                        .get_mut::<OutputWriter>()
                        .println(format!("{}: {}", binding, desc))?;
                }
            },
            Commands::Plugin { plugin_name } => {
                let plugin_name = plugin_name.join(" ");
                if plugin_name.len() == 0 {
                    states.get_mut::<OutputWriter>().println(format!(
                        "{} Plugins installed",
                        states.get::<PluginMetas>().len()
                    ))?;
                    for meta in states.get::<PluginMetas>().iter() {
                        states.get_mut::<OutputWriter>().println("")?;

                        states
                            .get_mut::<OutputWriter>()
                            .print_buf(styled_buf!(meta.name.clone().red()))?;
                        states.get_mut::<OutputWriter>().println("")?;
                        states
                            .get_mut::<OutputWriter>()
                            .println(meta.description.clone())?;
                    }
                } else {
                    let mut found = false;
                    for meta in states.get::<PluginMetas>().iter() {
                        if meta.name == plugin_name {
                            found = true;
                            states.get_mut::<OutputWriter>().println("")?;
                            states
                                .get_mut::<OutputWriter>()
                                .print_buf(styled_buf!(meta.name.clone().green()))?;
                            states.get_mut::<OutputWriter>().println("")?;
                            match &meta.help {
                                Some(help_text) => states
                                    .get_mut::<OutputWriter>()
                                    .println(help_text.clone())?,
                                None => states
                                    .get_mut::<OutputWriter>()
                                    .println("No help message provided.")?,
                            }
                            break;
                        }
                    }
                    if !found {
                        states.get_mut::<OutputWriter>().println("")?;
                        states.get_mut::<OutputWriter>().print_buf(
                            styled_buf!(format!(
                                "'{}' was not found, please specify a valid plugin name.",
                                plugin_name
                            ))
                            .red(),
                        )?;
                        states.get_mut::<OutputWriter>().println("")?;
                    }
                }
                states.get_mut::<OutputWriter>().println("")?;
            },
        }

        Ok(CmdOutput::success())
    }
}
