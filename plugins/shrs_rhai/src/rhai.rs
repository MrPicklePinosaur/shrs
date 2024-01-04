//! Library functions to interact with shrs from rhai scripts
use std::{cell::RefCell, rc::Rc};

use rhai::{Dynamic, Engine, ImmutableString, Scope};
use shrs::prelude::*;

pub struct RhaiState<'a> {
    pub engine: Engine,
    pub scope: Scope<'a>,
}

impl<'a> RhaiState<'a> {
    pub fn new(sh: &Shell, ctx: &mut Context, rt: &mut Runtime) -> Self {
        let mut engine = Engine::new();
        let rt = Rc::new(RefCell::new(rt.clone())); // TODO copying so changes will not persist

        {
            let rt = rt.clone();
            engine.register_fn("export_env", move |name: &str, value: &str| {
                let _ = rt.borrow_mut().env.set(name, value);
            });
        }
        {
            let rt = rt.clone();
            engine.register_fn("env", move |name: &str| -> String {
                rt.borrow().env.get(&name).cloned().unwrap_or(String::new())
            });
        }
        RhaiState {
            engine,
            scope: Scope::new(),
        }
    }
}
