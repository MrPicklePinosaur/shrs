//! Readline implementation for shrs
//!
//! Readline is the part of the shell that is responsible for taking in user input. It handles a
//! variety of things like keeping track of history, syntax highlighting, tab completion, vi mode,
//! and many more.
//!
//! shrs_line has a similar design philosophy to the rest of shrs in that it is also highly
//! configurable and extensible. Simply construct your own readline and give it to shrs to use.
//!

pub mod buffer_history;
pub mod highlight;
pub mod line_events;
pub mod line;
pub mod menu;
pub mod painter;
pub mod prompt;
pub mod snippet;
pub mod suggester;
pub mod vi;
