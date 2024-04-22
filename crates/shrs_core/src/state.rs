//! Globally accessible state store

use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::prelude::HookParam;

/// State store that uses types to index
impl<'res, T: 'static> HookParam for State<'res, T> {
    type Item<'new> = State<'new, T>;

    fn retrieve<'r>(states: &'r States) -> Self::Item<'r> {
        State {
            value: states.get_ref(&TypeId::of::<T>()).unwrap().borrow(),
            _marker: PhantomData,
        }
    }
}

impl<'res, T: 'static> HookParam for StateMut<'res, T> {
    type Item<'new> = StateMut<'new, T>;

    fn retrieve<'r>(resources: &'r States) -> Self::Item<'r> {
        StateMut {
            value: resources.get_ref(&TypeId::of::<T>()).unwrap().borrow_mut(),
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
    pub fn insert<R: 'static>(&mut self, res: R) {
        self.states
            .insert(TypeId::of::<R>(), RefCell::new(Box::new(res)));
    }
    pub fn get_ref(&mut self, t: &TypeId) -> Option<&RefCell<Box<dyn Any>>> {
        self.states.get(t)
    }
    pub fn get<T>(&mut self) -> Option<&T> {
        if let Some(s) = self.states.get(&TypeId::of::<T>()) {
            return s.borrow().downcast_ref();
        }
        None
    }
    pub fn get_mut<T>(&mut self) -> Option<&mut T> {
        if let Some(s) = self.states.get(&TypeId::of::<T>()) {
            return s.borrow().downcast_mut();
        }
        None
    }
}
