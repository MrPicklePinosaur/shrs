//! Readline implementation for shrs

#[macro_use]
extern crate derive_builder;

mod line;
pub use line::{Line, LineBuilder, LineBuilderError, LineMode};

mod cursor;
pub use cursor::{Cursor, DefaultCursor};

mod history;
pub use history::{DefaultHistory, History};

mod menu;
pub use menu::{DefaultMenu, Menu};

mod prompt;
pub use prompt::{DefaultPrompt, Prompt};

pub mod completion;
pub mod vi;

mod cursor_buffer;
mod painter;
pub use painter::StyledBuf;

mod highlight;
pub use highlight::{DefaultHighlighter, Highlighter};

#[cfg(test)]
mod tests {}
