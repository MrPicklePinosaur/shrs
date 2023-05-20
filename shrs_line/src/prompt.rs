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

fn default_styled_content(s: String) -> StyledContent<String> {
    StyledContent::new(ContentStyle::default(), s)
}
/// Valid types that can be passed to the styled macro
// cool since anyone can implement this trait to add something else that can be passed to this
// macro
pub trait StyledDisplay {
    fn to_string(&self) -> StyledContent<String>;
}
impl<T: ToString> StyledDisplay for Option<T> {
    fn to_string(&self) -> StyledContent<String> {
        let styled = self
            .as_ref()
            .to_owned()
            .map(|x| x.to_string())
            .unwrap_or_default();
        default_styled_content(styled)
    }
}
impl<T: ToString, E> StyledDisplay for Result<T, E> {
    fn to_string(&self) -> StyledContent<String> {
        let styled = self
            .as_ref()
            .to_owned()
            .map(|x| x.to_string())
            .unwrap_or_default();
        default_styled_content(styled)
    }
}
impl StyledDisplay for &str {
    fn to_string(&self) -> StyledContent<String> {
        default_styled_content(ToString::to_string(&self))
    }
}
impl StyledDisplay for String {
    fn to_string(&self) -> StyledContent<String> {
        default_styled_content(ToString::to_string(&self))
    }
}
// TODO this currently has incorrect offset
impl StyledDisplay for StyledBuf {
    fn to_string(&self) -> StyledContent<String> {
        default_styled_content(ToString::to_string(&self))
    }
}
impl StyledDisplay for StyledContent<String> {
    fn to_string(&self) -> StyledContent<String> {
        self.clone()
    }
}
impl StyledDisplay for StyledContent<&str> {
    fn to_string(&self) -> StyledContent<String> {
        default_styled_content(ToString::to_string(self.content()))
    }
}

// would technically like to make macro accept ToString but we want special behavior for option
// type

#[macro_export]
macro_rules! styled {
    ($($(@($($style:ident),*))? $part:expr),* $(,)*) => {{
        use $crate::{StyledBuf, StyledDisplay};
        use crossterm::style::{Stylize, StyledContent, ContentStyle};

        StyledBuf::from_iter(vec![
            $({
                // TODO this will probably return a pretty vague compiler error, if possible try to find
                // way to panic with decent message when the cast doesn't work
                let part: &dyn StyledDisplay = &$part;
                part.to_string()$($(.$style())*)?
            }),*
        ])
    }};
}

#[cfg(test)]
mod tests {

    use std::any::Any;

    #[test]
    fn styled_macro() {
        use crossterm::style::Stylize;
        println!("test {}", "lol".blue().reset());

        let styled_buf = styled! {
            @(red,bold) Some("lol"),
            "lol",
            String::from("lol"),
            "lol".blue(),
            styled! { "lol" }
        };
        println!("out {}", styled_buf);
    }
}
