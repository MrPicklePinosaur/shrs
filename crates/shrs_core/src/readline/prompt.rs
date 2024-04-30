//! Shell prompt

use std::marker::PhantomData;

use shrs_utils::{styled_buf, styled_buf::StyledBuf};

use super::super::state::Param;
use crate::prelude::{Shell, States};

pub trait PromptFn {
    fn prompt(&self, sh: &Shell, ctx: &States) -> StyledBuf;
}

/// Implement this trait to create your own prompt
pub struct Prompt {
    pub prompt_left: Box<dyn PromptFn>,
    pub prompt_right: Box<dyn PromptFn>,
}
impl Prompt {
    pub fn from_sides<I, J, R: PromptFn + 'static, L: PromptFn + 'static>(
        left_prompt: impl IntoPromptFn<I, PromptFn = L>,
        right_prompt: impl IntoPromptFn<J, PromptFn = R>,
    ) -> Self {
        Self {
            prompt_left: Box::new(left_prompt.into_prompt()),
            prompt_right: Box::new(right_prompt.into_prompt()),
        }
    }
}
impl Default for Prompt {
    fn default() -> Self {
        Prompt::from_sides(default_prompt_left, default_prompt_right)
    }
}

fn default_prompt_left(sh: &Shell) -> StyledBuf {
    styled_buf!("> ")
}

fn default_prompt_right(sh: &Shell) -> StyledBuf {
    styled_buf!()
}
pub trait IntoPromptFn<Input> {
    type PromptFn: PromptFn;
    fn into_prompt(self) -> Self::PromptFn;
}
pub struct FunctionPromptFn<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}
impl<F> PromptFn for FunctionPromptFn<(Shell,), F>
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
        impl<F, $($params: Param),+> PromptFn for FunctionPromptFn<($($params,)+), F>
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
impl<F> IntoPromptFn<()> for F
where
    for<'a, 'b> &'a F: Fn(&Shell) -> StyledBuf,
{
    type PromptFn = FunctionPromptFn<(Shell,), Self>;

    fn into_prompt(self) -> Self::PromptFn {
        FunctionPromptFn {
            f: self,
            marker: Default::default(),
        }
    }
}

macro_rules! impl_into_prompt {
    (
        $($params:ident),+
    ) => {
        impl<F, $($params: Param),+> IntoPromptFn<($($params,)*)> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell) ->StyledBuf+
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell)->StyledBuf
        {
            type PromptFn = FunctionPromptFn<($($params,)+), Self>;

            fn into_prompt(self) -> Self::PromptFn{
                FunctionPromptFn {
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
