//! Globally accessible state store

use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::prelude::{line::LineState, Runtime, Shell};

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
        StateMut {
            value: states.states.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
            _marker: PhantomData,
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
#[derive(Default)]
pub struct States {
    states: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}
impl States {
    pub fn insert<S: 'static>(&mut self, res: S) {
        self.states
            .insert(TypeId::of::<S>(), RefCell::new(Box::new(res)));
    }
    pub fn remove<S>() {}

    pub fn get_mut<S: 'static>(&self) -> RefMut<S> {
        let s = self
            .states
            .get(&TypeId::of::<S>())
            .expect("Value Missing")
            .borrow_mut();
        RefMut::map(s, |b| b.downcast_mut::<S>().unwrap())
    }
    pub fn get<S: 'static>(&self) -> Ref<S> {
        let s = self
            .states
            .get(&TypeId::of::<S>())
            .expect("Value Missing")
            .borrow();
        Ref::map(s, |b| b.downcast_ref::<S>().unwrap())
    }
    pub fn line(&self) -> RefMut<LineState> {
        self.get_mut::<LineState>()
    }
}
