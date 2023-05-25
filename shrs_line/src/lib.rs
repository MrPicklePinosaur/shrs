//! Readline implementation for shrs

#[macro_use]
extern crate derive_builder;

mod line;
pub use line::{Line, LineBuilder, LineBuilderError, LineCtx, LineMode};

mod cursor;
pub use cursor::CursorStyle;

mod history;
pub use history::{DefaultHistory, FileBackedHistory, History};

mod menu;
pub use menu::{DefaultMenu, Menu};

mod prompt;
pub use prompt::{DefaultPrompt, Prompt, *};

pub mod completion;
pub mod vi;

mod painter;
pub use painter::StyledBuf;

mod highlight;
pub use highlight::{DefaultHighlighter, Highlighter, RuleFn, SyntaxHighlighter, SyntaxTheme};

mod keybinding;
pub use keybinding::{parse_keybinding, DefaultKeybinding, Keybinding};

mod buffer_history;
pub use buffer_history::{BufferHistory, DefaultBufferHistory};

mod hooks;
pub use hooks::*;

#[cfg(test)]
mod tests {}
