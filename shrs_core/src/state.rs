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

    /// Get data or return default if not exist
    ///
    /// Also inserts default into state store to ensure future gets don't fail
    pub fn get_or_default<T: 'static + Default>(&mut self) -> &T {
        if !self.store.contains_key(&TypeId::of::<T>()) {
            self.insert(T::default());
        }

        // TODO i think this is safe?
        self.get::<T>().unwrap()
    }

    /// Get data or return default if not exist
    ///
    /// Also inserts default into state store to ensure future gets don't fail
    pub fn get_mut_or_default<T: 'static + Default>(&mut self) -> &mut T {
        if !self.store.contains_key(&TypeId::of::<T>()) {
            self.insert(T::default());
        }

        self.get_mut::<T>().unwrap()
    }
}
