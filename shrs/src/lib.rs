#[macro_use]
extern crate derive_builder;

pub use shrs_core::*;
pub use shrs_line as line;

mod shell;
pub use shell::*;

pub mod plugin;

pub mod crossterm {
    pub use crossterm::{
        style::{Print, Stylize},
        QueueableCommand,
    };
}

pub mod prelude {
    pub use shrs_core::{builtin::*, hooks::*, prompt::*, *};
    pub use shrs_line::{completion::*, *};

    pub use crate::{crossterm::*, plugin::*, shell::*};
}
