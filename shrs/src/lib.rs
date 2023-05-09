#[macro_use]
extern crate derive_builder;

pub use shrs_core::*;
pub use shrs_line as line;

mod shell;
pub use shell::*;

pub mod plugin;
