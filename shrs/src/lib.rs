#[macro_use]
extern crate derive_builder;

pub use shrs_core::*;

mod shell;
pub use shell::*;

pub mod plugin;
