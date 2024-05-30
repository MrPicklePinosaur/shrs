//! Shell runtime hooks
//!
//! Hooks are user provided functions that are called on a variety of events that occur in the
//! shell. Shrs defines specific events like [`StartupEvent`] - which runs on every boot, allowing you
//! to create a welcome message, or [`ChangeDirEvent`] - which runs when the working directory is
//! changed. It is also possible to define and emit your own custom events, which can then be
//! subscribed to anywhere else in the shell.
//!
//! An event that can be emitted at any time in the shell lifecycle, by either the
//! shell directly, or as a custom event used in a 3rd party plugin. It is then possible to
//! register a hook on this event to be ran every time the event occurs.
//! To define your own event, you must satisfy the [`HookEventMarker`] trait. An easy way to
//! do this is to use the [`HookEvent`] derive macro.
//! ```
//! # use shrs_core::prelude::*;
//! // Define an event
//! #[derive(HookEvent)]
//! pub struct MyEvent {
//!     pub foo: u32,
//!     pub bar: String,
//! }
//!
//! // Define a handler that uses this event
//! let my_handler = |event: &MyEvent| -> anyhow::Result<()> {
//!     println!("my hook ran! foo: {}, bar: {}", event.foo, event.bar);
//!     Ok(())
//! };
//!
//! // Register the hook during the shell initialization process
//! let mut hooks = Hooks::new();
//! hooks.insert(my_handler);
//! let myshell = ShellBuilder::default().with_hooks(hooks);
//! ```
//!
//! Then to emit the event, we can construct an instance of the event and call
//! [`Shell::run_hooks`] on [`Shell`]. Note that we must be in a handler (insert link) to be able to query for
//! [`Shell`].
//! ```ignore
//! # use shrs_core::prelude::*;
//! # #[derive(HookEvent)]
//! # pub struct MyEvent {
//! #     pub foo: u32,
//! #     pub bar: String,
//! # }

//! let my_event = MyEvent {
//!     foo: 42,
//!     bar: "eggplant".into(),
//! };
//! shell.run_hooks(my_event);
//!```
//!

pub mod events;

use std::marker::PhantomData;

use anyhow::Result;
use log::warn;

use crate::{
    all_the_tuples,
    prelude::{Shell, States},
    state::Param,
};

/// Marker trait a hook struct must implement
///
/// Can use the [`HookEvent`] derive macro to automatically implement this.
pub trait HookEventMarker: 'static + std::marker::Send + std::marker::Sync {}

/// Shell state containing all registered hooks
#[derive(Default)]
pub struct Hooks {
    hooks: anymap::Map,
}

impl Hooks {
    /// Initialize the Hooks state struct
    pub fn new() -> Self {
        Self {
            hooks: anymap::Map::new(),
        }
    }

    /// Emit an event of given type. All hook handlers that are registered to the hook type will be
    /// executed. However, no guarantees are made on the order these hook handlers are ran.
    // TODO currently this will abort if a hook fails, potentially introduce fail modes like
    // 'Best Effort' - run all hooks and report any failures
    // 'Pedantic' - abort on the first failed hook
    // TODO code example
    pub fn run<C: HookEventMarker>(&self, sh: &Shell, states: &States, c: &C) -> Result<()> {
        if let Some(hook_list) = self.get::<C>() {
            for hook in hook_list.iter() {
                if let Err(e) = hook.run(sh, states, c) {
                    let type_name = std::any::type_name::<C>();
                    warn!("failed to execute hook {e} of type {type_name}");
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    /// Register a new hook of given type
    ///
    /// Multiple hook handlers can be registered for a given type of hook. However, no guarantees
    /// are made on the order or priority hook handlers are ran.
    // TODO code example
    pub fn insert<I, C: HookEventMarker, S: Hook<C> + 'static>(
        &mut self,
        hook: impl IntoHook<I, C, Hook = S>,
    ) {
        let h = Box::new(hook.into_hook());
        match self.hooks.get_mut::<Vec<StoredHook<C>>>() {
            Some(hook_list) => {
                hook_list.push(h);
            },
            None => {
                // register any empty vector for the type
                self.hooks.insert::<Vec<StoredHook<C>>>(vec![h]);
            },
        };
    }

    /// Get all hooks associated with a Ctx type. If the hook of type has not been registered, a
    /// None will be returned.
    pub fn get<C: HookEventMarker>(&self) -> Option<&Vec<Box<dyn Hook<C>>>> {
        self.hooks.get::<Vec<StoredHook<C>>>()
    }
}

/// The handler function that can be registered to listen to an event of a given type
///
/// The type of the context the hook handler receives determines when it will be ran.
// TODO code example
pub trait Hook<C: HookEventMarker> {
    fn run(&self, sh: &Shell, states: &States, ctx: &C) -> Result<()>;
}

pub trait IntoHook<Input, C: HookEventMarker> {
    type Hook: Hook<C>;

    fn into_hook(self) -> Self::Hook;
}

pub struct FunctionHook<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

pub type StoredHook<C> = Box<dyn Hook<C>>;

impl<F, C: HookEventMarker> Hook<C> for FunctionHook<C, F>
where
    for<'a, 'b> &'a F: Fn(&C) -> Result<()>,
{
    fn run(&self, sh: &Shell, _states: &States, c: &C) -> Result<()> {
        fn call_inner<C: HookEventMarker>(
            f: impl Fn(&C) -> Result<()>,
            sh: &Shell,
            states: &C,
        ) -> Result<()> {
            f(&states)
        }

        call_inner(&self.f, sh, c)
    }
}

macro_rules! impl_hook{
    (
        $($params:ident),+
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, C:HookEventMarker,$($params: Param),+> Hook<C> for FunctionHook<($($params),+,C), F>
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&C ) ->Result<()>+
                    Fn( $(<$params as Param>::Item<'b>),+,&C )->Result<()>
        {
            fn run(&self, sh:&Shell,states: &States, c: &C)->Result<()> {
                fn call_inner<C:HookEventMarker,$($params),+>(
                    f: impl Fn($($params),+,&C)->Result<()>,
                    $($params: $params),+
                    ,states:&C
                ) ->Result<()>{
                    f($($params),+,&states)
                }

                $(
                    let $params = $params::retrieve(sh,states).unwrap();
                )+

                call_inner(&self.f, $($params),+,c)
            }
        }
    }
}

impl<F, C: HookEventMarker> IntoHook<(), C> for F
where
    for<'a, 'b> &'a F: Fn(&C) -> Result<()>,
{
    type Hook = FunctionHook<C, Self>;

    fn into_hook(self) -> Self::Hook {
        FunctionHook {
            f: self,
            marker: Default::default(),
        }
    }
}

impl<C: HookEventMarker, H: Hook<C>> IntoHook<H, C> for H {
    type Hook = H;

    fn into_hook(self) -> Self::Hook {
        self
    }
}

macro_rules! impl_into_hook {
    (
        $($params:ident),+
    ) => {
        impl<F, C:HookEventMarker,$($params: Param),+> IntoHook<($($params,)+),C> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&C ) ->Result<()>+
                    Fn( $(<$params as Param>::Item<'b>),+,&C )->Result<()>
        {
            type Hook = FunctionHook<($($params,)+C), Self>;

            fn into_hook(self) -> Self::Hook {
                FunctionHook {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}

all_the_tuples!(impl_hook, impl_into_hook);
