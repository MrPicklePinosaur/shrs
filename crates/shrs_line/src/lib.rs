//! Readline implementation for shrs
//!
//! Readline is the part of the shell that is responsible for taking in user input. It handles a
//! variety of things like keeping track of history, syntax highlighting, tab completion, vi mode,
//! and many more.
//!
//! shrs_line has a similar design philosophy to the rest of shrs in that it is also highly
//! configurable and extensible. Simply construct your own readline and give it to shrs to use.
//!
//! # Example
//! ```
//! use shrs_line::prelude::*;
//!
//! let mut myline = LineBuilder::default();
//! ```

#[macro_use]
extern crate derive_builder;

pub mod buffer_history;
pub mod completion;
pub mod cursor;
pub mod highlight;
pub mod hooks;
pub mod keybinding;
pub mod line;
pub mod menu;
pub mod painter;
pub mod prompt;
pub mod vi;

// TODO kinda ugly rexporting shrs_core here
pub use shrs_core as _core;

pub mod prelude {
    //! Imports the commonly used structs and types

    pub use crate::{
        buffer_history::{BufferHistory, DefaultBufferHistory},
        completion::*,
        cursor::CursorStyle,
        highlight::{DefaultHighlighter, Highlighter, RuleFn, SyntaxHighlighter, SyntaxTheme},
        hooks::*,
        keybinding::{parse_keybinding, BindingFn, DefaultKeybinding, Keybinding},
        line::{Line, LineBuilder, LineBuilderError, LineCtx, LineMode, Readline},
        menu::{DefaultMenu, Menu},
        painter::StyledBuf,
        prompt::{DefaultPrompt, Prompt, *},
        vi::*,
    };
    // Macros
    pub use crate::{keybindings, styled};
}

#[cfg(test)]
mod tests {}
