//! Globally accessible state store

use std::{
    any::{type_name, Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use thiserror::Error;

use crate::prelude::{line::LineContents, Runtime, Shell};

pub trait Param {
    type Item<'new>;

    fn retrieve<'r>(states: &'r States) -> Self::Item<'r>;
}

/// State store that uses types to index
impl<'res, T: 'static> Param for State<'res, T> {
    type Item<'new> = State<'new, T>;

    fn retrieve<'r>(states: &'r States) -> Self::Item<'r> {
        State {
            value: states.states.get(&TypeId::of::<T>()).unwrap().borrow(),
            _marker: PhantomData,
        }
    }
}

impl<'res, T: 'static> Param for StateMut<'res, T> {
    type Item<'new> = StateMut<'new, T>;

    fn retrieve<'r>(states: &'r States) -> Self::Item<'r> {
        let state = states.states.get(&TypeId::of::<T>());

        match state {
            Some(v) => StateMut {
                value: states.states.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
                _marker: PhantomData,
            },

            None => {
                panic!("{} Not Found", type_name::<T>())
            },
        }
    }
}

pub struct State<'a, T: 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a T>,
}

impl<T: 'static> Deref for State<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

pub struct StateMut<'a, T: 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}

impl<T: 'static> Deref for StateMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T: 'static> DerefMut for StateMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}

// Potential errors that can occur when interacting with state store
#[derive(Error, Debug)]
pub enum StateError {
    // TODO include the type in the error message
    #[error("Value is missing")]
    Missing,
    #[error("Failed to borrow")]
    Borrow,
    #[error("Failed to borrow mut")]
    BorrowMut,
    #[error("Failed to downcast")]
    Downcast,
}

// Global state store
#[derive(Default)]
pub struct States {
    states: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl States {

    // Insert a new piece of state of given type into global state store, overriding previously
    // existing values
    // TODO should we allow overriding contents of state?
    pub fn insert<S: 'static>(&mut self, res: S) {
        self.states
            .insert(TypeId::of::<S>(), RefCell::new(Box::new(res)));
    }

    // TODO this is potentially dangerous to allow arbitrary code to remove state
    pub fn remove<S>() -> Option<S> {
        todo!()
    }

    /// Get an immutable borrow of a state of a given type S from global state store. Will panic
    /// if a borrow exists or the type specified does not exist in the state store
    pub fn get<S: 'static>(&self) -> Ref<S> {
        self.try_get().unwrap()
    }

    /// Attempts to get an immutable borrow of a state of a given type S from global state store
    pub fn try_get<S: 'static>(&self) -> Result<Ref<S>, StateError> {
        let Some(s) = self
            .states
            .get(&TypeId::of::<S>()) else {

            return Err(StateError::Missing);
        };

        let Ok(s) = s.try_borrow() else {
            return Err(StateError::Borrow)
        };

        let Ok(s) = Ref::filter_map(s, |b| b.downcast_ref::<S>()) else {
            return Err(StateError::Downcast)
        };

        Ok(s)
    }

    /// Get a mutable borrow of a state of a given type S from global state store. Will panic
    /// if a borrow exists or the type specified does not exist in the state store
    pub fn get_mut<S: 'static>(&self) -> RefMut<S> {
        self.try_get_mut().unwrap()
    }

    /// Attempts to get a mutable borrow of a state of a given type S from global state store
    pub fn try_get_mut<S: 'static>(&self) -> Result<RefMut<S>, StateError> {
        let Some(s) = self
            .states
            .get(&TypeId::of::<S>()) else {

            return Err(StateError::Missing);
        };

        let Ok(s) = s.try_borrow_mut() else {
            return Err(StateError::Borrow)
        };

        let Ok(s) = RefMut::filter_map(s, |b| b.downcast_mut::<S>()) else {
            return Err(StateError::Downcast)
        };

        Ok(s)
    }
}
