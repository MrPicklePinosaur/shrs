//! Library functions to interact with shrs from rhai scripts
use std::{cell::RefCell, rc::Rc};

use rhai::{Dynamic, Engine, ImmutableString};
use shrs::prelude::*;

pub fn create_engine(sh: &Shell, ctx: &mut Context, rt: &mut Runtime) -> Engine {
    let mut engine = Engine::new();
    let rt = Rc::new(RefCell::new(rt.clone())); // TODO copying so changes will not persist

    // engine.register_fn("export", |name: ImmutableString, value: ImmutableString| {
    //     rt.env.set(&name, &value);
    // });
    engine.register_fn("env", move |name: ImmutableString| -> String {
        rt.borrow().env.get(&name).cloned().unwrap_or(String::new())
    });
    engine
}
