//! Builtin commands
//!
//! The main difference between builtin commands and external commands is that builtin commands
//! have access to the shell's context during execution. This may be useful if you specifically
//! need to query or mutate the shell's state. Some uses of this include switching the working
//! directory, calling hooks or accessing the state store.

mod alias;
mod cd;
mod debug;
mod exit;
mod export;
mod help;
mod history;
mod jobs;
mod source;
mod r#type;
mod unalias;

use std::{
    collections::{hash_map::Iter, HashMap},
    marker::PhantomData,
};

use anyhow::Result;

use self::{exit::exit_builtin, help::help_builtin};
use crate::{
    prelude::{CmdOutput, States},
    shell::Shell,
    state::Param,
};
// TODO could prob just be a map, to support arbitrary (user defined even) number of builtin commands
// just provide an easy way to override the default ones
/// Store for all registered builtin commands
pub struct Builtins {
    builtins: HashMap<String, Box<dyn Builtin>>,
}

// TODO a lot of this api is silly, perhaps just expose the entire hashmap
impl Builtins {
    /// Initializes a builtin container with no registered builtins
    ///
    /// You probably want to use `Builtins::default()` instead to get some sensible default
    /// builtins to use, then override the ones you want
    pub fn new() -> Self {
        Builtins {
            builtins: HashMap::new(),
        }
    }

    /// Insert a builtin command of the given name
    ///
    /// If a builtin of the same name has been registered, it will be overwritten.
    pub fn insert<I, B: Builtin + 'static>(
        &mut self,
        name: String,
        builtin: impl IntoBuiltin<I, Builtin = B>,
    ) {
        let item = Box::new(builtin.into_builtin());
        self.builtins.insert(name, item);
    }

    /// Get iterator of all registered builtin commands
    pub fn iter(&self) -> Iter<'_, String, Box<dyn Builtin>> {
        self.builtins.iter()
    }

    /// Find a builtin by name
    // Clippy thinks this shouldn't be a box, but it does not compile if you follow the warning
    #[allow(clippy::borrowed_box)]
    pub fn get(&self, name: &'static str) -> Option<&Box<dyn Builtin>> {
        self.builtins.get(name)
    }
}

impl Default for Builtins {
    fn default() -> Self {
        let mut b = Builtins::new();
        b.insert("exit".to_string(), exit_builtin);
        b.insert("help".to_string(), help_builtin);
        b

        // Builtins {
        //     builtins: HashMap::from([
        //         // (
        //         //     "history",
        //         //     Box::<HistoryBuiltin>::default() as Box<dyn Builtin>,
        //         // ),
        //         // ("exit", Box::<ExitBuiltin>::default() as Box<dyn Builtin>),
        //         // ("cd", Box::<CdBuiltin>::default() as Box<dyn Builtin>),
        //         // ("debug", Box::<DebugBuiltin>::default() as Box<dyn Builtin>),
        //         // (
        //         //     "export",
        //         //     Box::<ExportBuiltin>::default() as Box<dyn Builtin>,
        //         // ),
        //         // ("alias", Box::<AliasBuiltin>::default() as Box<dyn Builtin>),
        //         // (
        //         //     "unalias",
        //         //     Box::<UnaliasBuiltin>::default() as Box<dyn Builtin>,
        //         // ),
        //         // (
        //         //     "source",
        //         //     Box::<SourceBuiltin>::default() as Box<dyn Builtin>,
        //         // ),
        //         // ("type", Box::new(TypeBuiltin::default()) as Box<dyn Builtin>),
        //         // ("jobs", Box::<JobsBuiltin>::default() as Box<dyn Builtin>),
        //     ]),
        // }
    }
}

/// Implement this trait to define your own builtin command
pub trait Builtin {
    fn run(&self, sh: &Shell, states: &States, args: &Vec<String>) -> Result<CmdOutput>;
}
pub trait IntoBuiltin<Input> {
    type Builtin: Builtin;
    fn into_builtin(self) -> Self::Builtin;
}
pub struct FunctionBuiltin<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}
impl<F> Builtin for FunctionBuiltin<(Shell, Vec<String>), F>
where
    for<'a, 'b> &'a F: Fn(&Shell, &Vec<String>) -> Result<CmdOutput>,
{
    fn run(&self, sh: &Shell, ctx: &States, args: &Vec<String>) -> Result<CmdOutput> {
        fn call_inner(
            f: impl Fn(&Shell, &Vec<String>) -> Result<CmdOutput>,
            sh: &Shell,
            args: &Vec<String>,
        ) -> Result<CmdOutput> {
            f(&sh, &args)
        }

        call_inner(&self.f, sh, &args)
    }
}

macro_rules! impl_builtin {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, $($params: Param),+> Builtin for FunctionBuiltin<($($params,)+), F>
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell,&Vec<String>)->Result<CmdOutput> +
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell,&Vec<String> )->Result<CmdOutput>
        {
            fn run(&self, sh: &Shell,states: &States, args: &Vec<String>)->Result<CmdOutput> {
                fn call_inner<$($params),+>(
                    f: impl Fn($($params),+,&Shell,&Vec<String>)->Result<CmdOutput>,
                    $($params: $params),*
                    ,sh:&Shell,args:&Vec<String>
                ) -> Result<CmdOutput>{
                    f($($params),*,sh,args)
                }

                $(
                    let $params = $params::retrieve(states);
                )+

                call_inner(&self.f, $($params),+,sh,&args)
            }
        }
    }
}
impl<F> IntoBuiltin<()> for F
where
    for<'a, 'b> &'a F: Fn(&Shell, &Vec<String>) -> Result<CmdOutput>,
{
    type Builtin = FunctionBuiltin<(Shell, Vec<String>), Self>;

    fn into_builtin(self) -> Self::Builtin {
        FunctionBuiltin {
            f: self,
            marker: Default::default(),
        }
    }
}

macro_rules! impl_into_builtin {
    (
        $($params:ident),+
    ) => {
        impl<F, $($params: Param),+> IntoBuiltin<($($params,)*)> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell,&Vec<String> ) ->Result<CmdOutput>+
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell,&Vec<String> )->Result<CmdOutput>
        {
            type Builtin = FunctionBuiltin<($($params,)+), Self>;

            fn into_builtin(self) -> Self::Builtin {
                FunctionBuiltin {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}
impl_builtin!(T1);
impl_builtin!(T1, T2);
impl_builtin!(T1, T2, T3);
impl_builtin!(T1, T2, T3, T4);
impl_into_builtin!(T1);
impl_into_builtin!(T1, T2);
impl_into_builtin!(T1, T2, T3);
impl_into_builtin!(T1, T2, T3, T4);
