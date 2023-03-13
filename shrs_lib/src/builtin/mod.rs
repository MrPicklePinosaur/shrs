mod alias;
mod cd;
mod debug;
mod exit;
mod export;
mod history;
mod unalias;

use std::{collections::HashMap, process::Child};

use self::{
    alias::AliasBuiltin, cd::CdBuiltin, debug::DebugBuiltin, exit::ExitBuiltin,
    export::ExportBuiltin, history::HistoryBuiltin, unalias::UnaliasBuiltin,
};
use crate::shell::{Context, Runtime};

macro_rules! hashmap (
    { $($key:expr => $value:expr),+ } => {
	{
	    let mut m = std::collections::HashMap::new();
	    $(
		m.insert($key, $value);
	    )+
	    m
	}
    };
);

// TODO could prob just be a map, to support arbritrary (user defined even) number of builtin commands
// just provide an easy way to override the default ones
pub struct Builtins {
    pub builtins: HashMap<&'static str, Box<dyn BuiltinCmd>>,
}

impl Default for Builtins {
    fn default() -> Self {
        Builtins {
            builtins: HashMap::from([
                (
                    "history",
                    Box::new(HistoryBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
                (
                    "exit",
                    Box::new(ExitBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
                ("cd", Box::new(CdBuiltin::default()) as Box<dyn BuiltinCmd>),
                (
                    "debug",
                    Box::new(DebugBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
                (
                    "export",
                    Box::new(ExportBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
                (
                    "alias",
                    Box::new(AliasBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
                (
                    "unalias",
                    Box::new(UnaliasBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
            ]),
        }
    }
}

pub trait BuiltinCmd {
    fn run(&self, ctx: &mut Context, rt: &mut Runtime, args: &Vec<String>)
        -> anyhow::Result<Child>;
}
