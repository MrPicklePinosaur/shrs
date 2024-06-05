//! Globally accessible state store
//!
//! States is a collection that holds are shared mutable data
//! The values within state are accessible in handler using `State` and `StateMut` params
//! Inserting and removing states can be done using Commands

use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use anyhow::{anyhow, Context, Result};
use thiserror::Error;

use crate::prelude::Shell;
/// Trait needs to be implemented for all parameters of handlers
pub trait Param {
    type Item<'new>;

    fn retrieve<'r>(shell: &'r Shell, states: &'r States) -> Result<Self::Item<'r>>;
}

// Result is not necessary to be implemented
// impl<T: 'static> Param for Result<T>
// where
//     T: Param,
//     <T as Param>::Item<'static>: 'static,
// {
//     type Item<'new> = Result<<T as Param>::Item<'new>>;

//     fn retrieve<'r>(shell: &'r Shell, states: &'r States) -> Result<Self::Item<'r>> {
//         Ok(T::retrieve(shell, states))
//     }
// }

/// Option wrapper for [`Param`]
/// ```
/// # use shrs_core::prelude::*;
/// fn s(mut out: Option<StateMut<OutputWriter>>)-> anyhow::Result<()>{
///     if let Some(mut o) = out{
///         o.println("o exists")?
///     }
///     Ok(())
/// }
/// ```
/// enables [`Option`] to be wrapped around any other [`Param`]
/// If the state does not exist, `Option<T>` will be None
impl<T: 'static> Param for Option<T>
where
    T: Param,
    <T as Param>::Item<'static>: 'static,
{
    type Item<'new> = Option<<T as Param>::Item<'new>>;

    fn retrieve<'r>(shell: &'r Shell, states: &'r States) -> Result<Self::Item<'r>> {
        Ok(T::retrieve(shell, states).ok())
    }
}
/// Implementation of [`Param`] for accessing [`Shell`]
/// ```
/// # use shrs_core::prelude::*;
/// fn s(sh: &Shell)-> anyhow::Result<()>{
///
///     Ok(())
/// }
/// ```
/// [`Shell`] can only be accessed immutably. To mutate it, use [`crate::prelude::Commands`]
impl<'res> Param for &'res Shell {
    type Item<'new> = &'new Shell;

    fn retrieve<'r>(shell: &'r Shell, _states: &'r States) -> Result<Self::Item<'r>> {
        Ok(shell)
    }
}
/// Wrapper for accessing a state from [`States`] immutably
/// ```
/// # use shrs_core::prelude::*;
/// fn s(rt: State<Runtime>)-> anyhow::Result<()>{
///     //rt can now be automatically dereferenced to get `&Runtime`
///     let x = rt.working_dir.clone();
///     Ok(())
/// }
/// ```
/// If the state does not exist, handler will panic
pub struct State<'a, T: 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a T>,
}
impl<'res, T: 'static> Param for State<'res, T> {
    type Item<'new> = State<'new, T>;

    fn retrieve<'r>(_shell: &'r Shell, states: &'r States) -> Result<Self::Item<'r>> {
        Ok(State {
            value: states
                .states
                .get(&TypeId::of::<T>())
                .context("State Not Found")?
                .borrow(),
            _marker: PhantomData,
        })
    }
}

impl<T: 'static> Deref for State<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}
/// Wrapper for accessing a state from [`States`] mutably
/// ```
/// # use shrs_core::prelude::*;
/// // mut is required
/// fn s(mut out: StateMut<OutputWriter>)-> anyhow::Result<()>{
///     //out can now be automatically dereferenced to get `&mut OutputWriter`
///     out.println("Hello")
/// }
/// ```
/// If the state does not exist, handler will panic
pub struct StateMut<'a, T: 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}
impl<'res, T: 'static> Param for StateMut<'res, T> {
    type Item<'new> = StateMut<'new, T>;

    fn retrieve<'r>(_shell: &'r Shell, states: &'r States) -> Result<Self::Item<'r>> {
        Ok(StateMut {
            value: states
                .states
                .get(&TypeId::of::<T>())
                .context("State Not Found")?
                .borrow_mut(),
            _marker: PhantomData,
        })
    }
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

/// Potential errors that can occur when interacting with state store
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

/// Global state store
///
/// The values are stored in [`RefCell`] due to borrowing limitations placed due to the DI implementation
/// When using States directly, need to be careful of borrowing rules since [`RefCell`] panics instead of being a compile time error.
#[derive(Default)]
pub struct States {
    states: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl States {
    /// Insert a new piece of state of given type into global state store, overriding previously
    /// existing values
    // Overrides contents of existing state, if there is any
    pub fn insert<S: 'static>(&mut self, res: S) {
        self.states
            .insert(TypeId::of::<S>(), RefCell::new(Box::new(res)));
    }

    // TODO this is potentially dangerous to allow arbitrary code to remove state
    pub fn remove<S: 'static>(&mut self) -> Result<(), StateError> {
        let Some(s) = self.states.remove(&TypeId::of::<S>()) else {
            return Err(StateError::Missing);
        };
        Ok(())
    }

    /// Get an immutable borrow of a state of a given type S from global state store. Will panic
    /// if a borrow exists or the type specified does not exist in the state store
    pub fn get<S: 'static>(&self) -> Ref<S> {
        self.try_get().unwrap()
    }

    /// Attempts to get an immutable borrow of a state of a given type S from global state store
    pub fn try_get<S: 'static>(&self) -> Result<Ref<S>, StateError> {
        let Some(s) = self.states.get(&TypeId::of::<S>()) else {
            return Err(StateError::Missing);
        };

        let Ok(s) = s.try_borrow() else {
            return Err(StateError::Borrow);
        };

        let Ok(s) = Ref::filter_map(s, |b| b.downcast_ref::<S>()) else {
            return Err(StateError::Downcast);
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
        let Some(s) = self.states.get(&TypeId::of::<S>()) else {
            return Err(StateError::Missing);
        };

        let Ok(s) = s.try_borrow_mut() else {
            return Err(StateError::Borrow);
        };

        let Ok(s) = RefMut::filter_map(s, |b| b.downcast_mut::<S>()) else {
            return Err(StateError::Downcast);
        };

        Ok(s)
    }
}
