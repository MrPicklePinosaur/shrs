//! Shell prompt
//!
//! Prompts can be customized and built with various styled custom components and many functions,
//! such as git branch, exit status of previous command, battery level, weather, you name it!
//!
//! A prompt consists of a left and right part and each part is allowed to span multiple lines.
//! Each part of the prompt is just a function that returns a [`StyledBuf`] object. See docs for
//! [`StyledBuf`] for more details. The prompt function is also a handler (insert link). This
//! means that it is able to query for shell state. In the example below we fetch the
//! [`crate::readline::line::LineMode`], which tells us if we are in 'insert' mode or 'normal' mode, and we change the
//! prompt style depending on the mode.
//! ```
//! # use shrs_core::prelude::*;
//! # use shrs_utils::*;
//! pub use crossterm::style::Stylize;
//!
//! // Simple prompt that displays username, working directory, and a vi mode indicator
//! fn prompt_left(line_mode: State<LineMode>) -> StyledBuf {
//!     // Display which vi mode we are in
//!     let indicator = match *line_mode {
//!         LineMode::Insert => String::from(">").cyan(),
//!         LineMode::Normal => String::from(":").yellow(),
//!     };
//!
//!     styled_buf!(
//!         " ",
//!         username().map(|u| u.blue()),
//!         " ",
//!         top_pwd().white().bold(),
//!         " ",
//!         indicator,
//!         " "
//!     )
//! }
//!
//! // Dummy right prompt
//! fn prompt_right() -> StyledBuf {
//!     styled_buf!()
//! }
//!
//! // Register the prompt with the shell
//! let prompt = Prompt::from_sides(prompt_left, prompt_right);
//! let myshell = ShellBuilder::default().with_prompt(prompt);
//! ```
//!

mod utils;
use std::marker::PhantomData;

use crossterm::style::Stylize;
use shrs_utils::{styled_buf, StyledBuf};
pub use utils::*;

use super::super::state::*;
use crate::prelude::{Shell, States};

/// Handler for either side of the prompt.
pub trait PromptFn {
    fn prompt(&self, sh: &Shell, ctx: &States) -> StyledBuf;
}

/// [`Prompt`] is split into right and left where each is a [`PromptFn`]
pub struct Prompt {
    pub prompt_left: Box<dyn PromptFn>,
    pub prompt_right: Box<dyn PromptFn>,
}

impl Prompt {
    /// Constructor for making a [`Prompt`] from two [`PromptFn`
    pub fn from_sides<I, J, R: PromptFn + 'static, L: PromptFn + 'static>(
        prompt_left: impl IntoPromptFn<I, PromptFn = L>,
        prompt_right: impl IntoPromptFn<J, PromptFn = R>,
    ) -> Self {
        Self {
            prompt_left: Box::new(prompt_left.into_prompt()),
            prompt_right: Box::new(prompt_right.into_prompt()),
        }
    }
    pub fn from_left<I, L: PromptFn + 'static>(
        prompt_left: impl IntoPromptFn<I, PromptFn = L>,
    ) -> Self {
        Self {
            prompt_left: Box::new(prompt_left.into_prompt()),
            prompt_right: Box::new((|| StyledBuf::empty()).into_prompt()),
        }
    }
    pub fn from_right<I, R: PromptFn + 'static>(
        prompt_right: impl IntoPromptFn<I, PromptFn = R>,
    ) -> Self {
        Self {
            prompt_left: Box::new((|| StyledBuf::empty()).into_prompt()),
            prompt_right: Box::new(prompt_right.into_prompt()),
        }
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt::from_left(default_prompt_left)
    }
}

/// Left side of bash style default prompt
fn default_prompt_left() -> StyledBuf {
    styled_buf!(" ", top_pwd().white().bold(), " > ")
}

pub trait IntoPromptFn<Input> {
    type PromptFn: PromptFn;
    fn into_prompt(self) -> Self::PromptFn;
}

pub struct FunctionPromptFn<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

macro_rules! impl_prompt {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, $($params: Param),*> PromptFn for FunctionPromptFn<($($params,)*), F>
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),*)->StyledBuf +
                    Fn( $(<$params as Param>::Item<'b>),*)->StyledBuf
        {
            fn prompt(&self, sh: &Shell,states: &States)->StyledBuf {
                fn call_inner<$($params),*>(
                    f: impl Fn($($params),*)->StyledBuf,
                    $($params: $params),*
                ) -> StyledBuf{
                    f($($params),*)
                }

                $(
                    let $params = $params::retrieve(sh,states).unwrap();
                )*

                call_inner(&self.f, $($params),*)
            }
        }
    }
}

macro_rules! impl_into_prompt {
    (
        $($params:ident),*
    ) => {
        impl<F, $($params: Param),*> IntoPromptFn<($($params,)*)> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),*) ->StyledBuf+
                    Fn( $(<$params as Param>::Item<'b>),*)->StyledBuf
        {
            type PromptFn = FunctionPromptFn<($($params,)*), Self>;

            fn into_prompt(self) -> Self::PromptFn{
                FunctionPromptFn {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}
impl_prompt!();
impl_into_prompt!();
all_the_tuples!(impl_prompt, impl_into_prompt);
