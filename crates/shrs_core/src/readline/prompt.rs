//! Shell prompt

use std::marker::PhantomData;

use shrs_utils::{styled_buf, styled_buf::StyledBuf};

use super::super::state::Param;
use crate::prelude::{Shell, States};

/// Implement this trait to create your own prompt
pub trait Prompt {
    fn prompt(&self, sh: &Shell, ctx: &States) -> StyledBuf;
}

pub struct FullPrompt {
    pub prompt_left: Box<dyn Prompt>,
    pub prompt_right: Box<dyn Prompt>,
}
impl FullPrompt {
    pub fn from_sides<I, J, R: Prompt + 'static, L: Prompt + 'static>(
        left_prompt: impl IntoPrompt<I, Prompt = L>,
        right_prompt: impl IntoPrompt<J, Prompt = R>,
    ) -> Self {
        Self {
            prompt_left: Box::new(left_prompt.into_prompt()),
            prompt_right: Box::new(right_prompt.into_prompt()),
        }
    }
}
impl Default for FullPrompt {
    fn default() -> Self {
        FullPrompt::from_sides(default_prompt_left, default_prompt_right)
    }
}

fn default_prompt_left(sh: &Shell) -> StyledBuf {
    styled_buf!("> ")
}

fn default_prompt_right(sh: &Shell) -> StyledBuf {
    styled_buf!()
}
pub trait IntoPrompt<Input> {
    type Prompt: Prompt;
    fn into_prompt(self) -> Self::Prompt;
}
pub struct FunctionPrompt<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}
impl<F> Prompt for FunctionPrompt<(Shell,), F>
where
    for<'a, 'b> &'a F: Fn(&Shell) -> StyledBuf,
{
    fn prompt(&self, sh: &Shell, ctx: &States) -> StyledBuf {
        fn call_inner(f: impl Fn(&Shell) -> StyledBuf, sh: &Shell) -> StyledBuf {
            f(&sh)
        }

        call_inner(&self.f, sh)
    }
}

macro_rules! impl_prompt {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, $($params: Param),+> Prompt for FunctionPrompt<($($params,)+), F>
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell)->StyledBuf +
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell)->StyledBuf
        {
            fn prompt(&self, sh: &Shell,states: &States)->StyledBuf {
                fn call_inner<$($params),+>(
                    f: impl Fn($($params),+,&Shell)->StyledBuf,
                    $($params: $params),*
                    ,sh:&Shell
                ) -> StyledBuf{
                    f($($params),*,sh)
                }

                $(
                    let $params = $params::retrieve(states);
                )+

                call_inner(&self.f, $($params),+,sh)
            }
        }
    }
}
impl<F> IntoPrompt<()> for F
where
    for<'a, 'b> &'a F: Fn(&Shell) -> StyledBuf,
{
    type Prompt = FunctionPrompt<(Shell,), Self>;

    fn into_prompt(self) -> Self::Prompt {
        FunctionPrompt {
            f: self,
            marker: Default::default(),
        }
    }
}

macro_rules! impl_into_prompt {
    (
        $($params:ident),+
    ) => {
        impl<F, $($params: Param),+> IntoPrompt<($($params,)*)> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell) ->StyledBuf+
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell)->StyledBuf
        {
            type Prompt = FunctionPrompt<($($params,)+), Self>;

            fn into_prompt(self) -> Self::Prompt {
                FunctionPrompt {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}
impl_prompt!(T1);
impl_prompt!(T1, T2);
impl_prompt!(T1, T2, T3);
impl_prompt!(T1, T2, T3, T4);
impl_into_prompt!(T1);
impl_into_prompt!(T1, T2);
impl_into_prompt!(T1, T2, T3);
impl_into_prompt!(T1, T2, T3, T4);
