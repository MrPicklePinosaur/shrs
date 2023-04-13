//! Globally accessable state store

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct State {
    store: HashMap<TypeId, Box<dyn Any>>,
}

impl State {
    pub fn new() -> State {
        State {
            store: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, data: T) {
        self.store.insert(TypeId::of::<T>(), Box::new(data));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.store
            .get(&TypeId::of::<T>())
            .and_then(|data_any| data_any.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.store
            .get_mut(&TypeId::of::<T>())
            .and_then(|data_any| data_any.downcast_mut::<T>())
    }
}
