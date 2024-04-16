//! Library functions to interact with shrs from rhai scripts
use std::collections::HashMap;

use rhai::{Engine, Scope, AST};

pub struct RhaiState<'a> {
    pub engine: Engine,
    pub scope: Scope<'a>,
    /// Store previously evaluated AST
    pub ast: HashMap<String, AST>,
}

impl<'a> RhaiState<'a> {
    pub fn new() -> Self {
        let engine = Engine::new();

        RhaiState {
            engine,
            scope: Scope::new(),
            ast: HashMap::new(),
        }
    }
}
