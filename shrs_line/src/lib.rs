//! Readline implementation for shrs

#[macro_use]
extern crate derive_builder;

mod line;
pub use line::{Line, LineBuilder, LineBuilderError, LineCtx, LineMode};

mod cursor;
pub use cursor::{Cursor, DefaultCursor};

mod history;
pub use history::{DefaultHistory, FileBackedHistory, History};

mod menu;
pub use menu::{DefaultMenu, Menu};

mod prompt;
pub use prompt::{DefaultPrompt, Prompt};

pub mod completion;
pub mod vi;

mod painter;
pub use painter::StyledBuf;

mod highlight;
pub use highlight::{DefaultHighlighter, Highlighter};

mod keybinding;
pub use keybinding::{DefaultKeybinding, Keybinding};

mod buffer_history;
pub use buffer_history::{BufferHistory, DefaultBufferHistory};

#[cfg(test)]
mod tests {}
