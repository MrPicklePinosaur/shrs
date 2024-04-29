//! Library functions to interact with shrs from rhai scripts
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use pino_deref::{Deref, DerefMut};
use rhai::{Engine, Scope, AST};

pub struct RhaiScope<'a>(pub Scope<'a>);

impl<'a> RhaiScope<'a> {
    pub fn new() -> Self {
        RhaiScope(Scope::new())
    }
}

// TODO can use macro when pino_deref supports lifetimes
impl<'a> Deref for RhaiScope<'a> {
    type Target = Scope<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for RhaiScope<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Deref, DerefMut)]
pub struct RhaiAST(HashMap<String, AST>);

impl RhaiAST {
    pub fn new() -> Self {
        RhaiAST(HashMap::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct RhaiEngine(pub Engine);

impl RhaiEngine {
    pub fn new() -> Self {
        RhaiEngine(Engine::new())
    }
}
