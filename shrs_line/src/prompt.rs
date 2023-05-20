//! Shell prompt

use std::{any::Any, fmt::Display};

use crossterm::style::{ContentStyle, StyledContent, Stylize};
use shrs_core::{Context, Runtime, Shell};
use thiserror::__private::DisplayAsDisplay;

use crate::{line::LineCtx, painter::StyledBuf};

pub trait Prompt {
    fn prompt_left(&self, line_ctx: &mut LineCtx) -> StyledBuf;
    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf;
}

/// Default implementation for [Prompt]
pub struct DefaultPrompt {}

impl DefaultPrompt {
    pub fn new() -> Self {
        DefaultPrompt {}
    }
}

impl Prompt for DefaultPrompt {
    // TODO i still don't like passing all this context down
    fn prompt_left(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        StyledBuf::from_iter(vec![StyledContent::new(
            ContentStyle::new(),
            "> ".to_string(),
        )])
    }

    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        StyledBuf::from_iter(vec![])
    }
}

/// Valid types that can be passed to the styled macro
// cool since anyone can implement this trait to add something else that can be passed to this
// macro
pub trait StyledDisplay {
    fn to_string(&self) -> String;
}
impl<T: ToString> StyledDisplay for Option<T> {
    fn to_string(&self) -> String {
        self.as_ref()
            .to_owned()
            .map(|x| x.to_string())
            .unwrap_or_default()
    }
}
impl StyledDisplay for &str {
    fn to_string(&self) -> String {
        ToString::to_string(&self)
    }
}
impl StyledDisplay for String {
    fn to_string(&self) -> String {
        ToString::to_string(&self)
    }
}
impl StyledDisplay for StyledBuf {
    fn to_string(&self) -> String {
        ToString::to_string(&self)
    }
}
impl<T: Display> StyledDisplay for StyledContent<T> {
    fn to_string(&self) -> String {
        ToString::to_string(&self)
    }
}

// would technically like to make macro accept ToString but we want special behavior for option
// type

#[macro_export]
macro_rules! styled {
    ($($part:expr),* $(,)*) => {{
        use $crate::{StyledBuf, StyledDisplay};
        use crossterm::style::Stylize;

        StyledBuf::from_iter(vec![
            $({
                // TODO this will probably return a pretty vague compiler error, if possible try to find
                // way to panic with decent message when the cast doesn't work
                let part: &dyn StyledDisplay = &$part;
                part.to_string().reset()
            }),*
        ])
    }}
}

#[cfg(test)]
mod tests {

    use std::any::Any;

    #[test]
    fn styled_macro() {
        use crossterm::style::Stylize;

        let styled_buf = styled! {
            Some("lol"),
            "lol",
            String::from("lol"),
            "lol".blue(),
            styled! { "lol" }
        };
        println!("out {}", styled_buf);
    }
}
