//! Readline implementation for shrs

#[macro_use]
extern crate derive_builder;

pub mod completion;
pub mod cursor;
pub mod cursor_buffer;
pub mod history;
pub mod line;
pub mod menu;
pub mod painter;
pub mod prompt;

#[cfg(test)]
mod tests {}
