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

pub mod buffer_history;
pub mod cursor;
pub mod highlight;
pub mod hooks;
pub mod line;
pub mod menu;
pub mod painter;
pub mod prompt;
pub mod vi;

pub use buffer_history::{BufferHistory, DefaultBufferHistory};
pub use cursor::CursorStyle;
pub use highlight::{DefaultHighlighter, Highlighter, SyntaxHighlighter, SyntaxTheme};
pub use hooks::*;
pub use line::{Line, LineBuilder, LineBuilderError, LineStateBundle, LineMode, Readline};
pub use menu::{DefaultMenu, Menu};
pub use prompt::{DefaultPrompt, Prompt, *};
pub use vi::*;
