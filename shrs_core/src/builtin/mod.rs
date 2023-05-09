mod alias;
mod cd;
mod debug;
mod exit;
mod export;
mod help;
mod history;
mod jobs;
mod source;
mod unalias;

use std::{
    collections::{hash_map::Iter, HashMap},
    process::Child,
};

use self::{
    alias::AliasBuiltin, cd::CdBuiltin, debug::DebugBuiltin, exit::ExitBuiltin,
    export::ExportBuiltin, help::HelpBuiltin, history::HistoryBuiltin, jobs::JobsBuiltin,
    source::SourceBuiltin, unalias::UnaliasBuiltin,
};
use crate::{
    shell::{Context, Runtime},
    Shell,
};

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
    builtins: HashMap<&'static str, Box<dyn BuiltinCmd>>,
}

impl Builtins {
    pub fn new() -> Self {
        Builtins {
            builtins: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &'static str, builtin: impl BuiltinCmd + 'static) {
        self.builtins.insert(name, Box::new(builtin));
    }

    pub fn iter(&self) -> Iter<'_, &str, Box<dyn BuiltinCmd>> {
        self.builtins.iter()
    }
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
                (
                    "source",
                    Box::new(SourceBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
                (
                    "jobs",
                    Box::new(JobsBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
                (
                    "help",
                    Box::new(HelpBuiltin::default()) as Box<dyn BuiltinCmd>,
                ),
            ]),
        }
    }
}

pub trait BuiltinCmd {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<Child>;
}
