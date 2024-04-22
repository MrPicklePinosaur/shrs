//! Shell runtime hooks
//!
//! Hooks are user provided functions that are called on a variety of events that occur in the
//! shell. Some additional context is provided to these hooks.
// ideas for hooks
// - on start
// - after prompt
// - before prompt
// - internal error hook (call whenever there is internal shell error; good for debug)
// - env hook (when environment variable is set/changed)
// - exit hook (tricky, make sure we know what cases to call this)

use std::marker::PhantomData;

use anyhow::Result;

use crate::{ctx::Ctx, state::States};

macro_rules! impl_hook{
    (
        $($params:ident),+
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, C:Ctx,$($params: HookParam),+> Hook<C> for FunctionHook<($($params,)*C), F>
            where
                for<'a, 'b> &'a mut F:
                    FnMut( $($params),+,&C ) ->Result<()>+
                    FnMut( $(<$params as HookParam>::Item<'b>),+,&C )->Result<()>
        {
            fn run(&mut self, states: &mut States, ctx: &C)->Result<()> {
                fn call_inner<C:Ctx,$($params),+>(
                    mut f: impl FnMut($($params),+,&C)->Result<()>,
                    $($params: $params),*
                    ,ctx:&C
                ) ->Result<()>{
                    f($($params),*,&ctx)
                }

                $(
                    let $params = $params::retrieve(states);
                )*

                call_inner(&mut self.f, $($params),+,ctx)
            }
        }
    }
}

macro_rules! impl_into_hook {
    (
        $($params:ident),*
    ) => {
        impl<F, C:Ctx,$($params: HookParam),*> IntoHook<($($params,)*),C> for F
            where
                for<'a, 'b> &'a mut F:
                    FnMut( $($params),+,&C ) ->Result<()>+
                    FnMut( $(<$params as HookParam>::Item<'b>),+,&C )->Result<()>
        {
            type Hook = FunctionHook<($($params,)+C), Self>;

            fn into_system(self) -> Self::Hook {
                FunctionHook {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}

pub trait HookParam {
    type Item<'new>;

    fn retrieve<'r>(states: &'r States) -> Self::Item<'r>;
}

struct FunctionHook<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

pub trait Hook<C: Ctx> {
    fn run(&mut self, resources: &mut States, ctx: &C) -> Result<()>;
}

impl_hook!(T1);
impl_hook!(T1, T2);
impl_hook!(T1, T2, T3);
impl_hook!(T1, T2, T3, T4);

trait IntoHook<Input, C: Ctx> {
    type Hook: Hook<C>;

    fn into_system(self) -> Self::Hook;
}

impl_into_hook!(T1);
impl_into_hook!(T1, T2);
impl_into_hook!(T1, T2, T3);
impl_into_hook!(T1, T2, T3, T4);

type StoredHook<C> = Box<dyn Hook<C>>;
#[derive(Default)]
pub struct Hooks {
    hooks: anymap::Map,
}
impl Hooks {
    pub fn new() -> Self {
        Self {
            hooks: anymap::Map::new(),
        }
    }

    pub fn insert<I, C: Ctx, S: Hook<C> + 'static>(
        &mut self,
        system: impl IntoHook<I, C, Hook = S>,
    ) {
        let item = Box::new(system.into_system());
        match self.hooks.get_mut::<Vec<StoredHook<C>>>() {
            Some(hook_list) => {
                hook_list.push(item);
            },
            None => {
                // register any empty vector for the type
                self.hooks.insert::<Vec<StoredHook<C>>>(vec![item]);
            },
        };
    }
    pub fn get_mut<C: Ctx>(&mut self) -> Option<&mut Vec<Box<dyn Hook<C>>>> {
        self.hooks.get_mut::<Vec<StoredHook<C>>>()
    }
}
