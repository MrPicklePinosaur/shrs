//! Library functions to interact with shrs from rhai scripts
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rhai::{Dynamic, Engine, ImmutableString, Scope, AST};
use shrs::prelude::*;

pub struct RhaiState<'a> {
    pub engine: Engine,
    pub scope: Scope<'a>,
    /// Store previously evaluated AST
    pub ast: HashMap<String, AST>,
}

impl<'a> RhaiState<'a> {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        RhaiState {
            engine,
            scope: Scope::new(),
            ast: HashMap::new(),
        }
    }
}
