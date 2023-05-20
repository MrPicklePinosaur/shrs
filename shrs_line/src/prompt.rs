//! Shell prompt

use std::{any::Any, fmt::Display};

use crossterm::style::{ContentStyle, StyledContent};
use shrs_core::{Context, Runtime, Shell};

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

pub trait StyledOption {
    fn to_string(&self) -> String;
    fn tester(&self);
}
impl<T: ToString + ToOwned> StyledOption for Option<T> {
    fn to_string(&self) -> String {
        self.as_ref()
            .to_owned()
            .map(|x| x.to_string())
            .unwrap_or_default()
    }
    fn tester(&self) {}
}

#[macro_export]
macro_rules! styled {
    ($part:expr) => {{
        use $crate::{StyledBuf, StyledOption};
        use std::any::Any;
        use crossterm::style::Stylize;

        let part_any = &$part as &dyn Any;
        let display = if let Some(x) = part_any.downcast_ref::<&dyn StyledOption>() {
            x.to_string()
        } else {
            panic!("unsupported type")
        };

        StyledBuf::from_iter(vec![
            // display.reset()
        ])
    }}
}

#[cfg(test)]
mod tests {

    use std::any::Any;

    use super::StyledOption;

    #[test]
    fn styled_macro() {
        let styled_buf = styled! {
            Some("lol")
        };
        println!("out {}", styled_buf);
    }
}
