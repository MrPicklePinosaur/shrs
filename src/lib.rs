#![doc(html_root_url = "https://docs.rs/shrs/0.0.1")]

//! **shrs** is a framework for building and configuring your own shell in rust.
//!
//! To get started with a simple shell with all the default options
//! ```
//! use shrs::prelude::*;
//!
//! fn main() {
//!     let myshell = ShellBuilder::default()
//!         .build()
//!         .unwrap();
//!
//!     myshell.run().unwrap();
//! }
//! ```
//! For more advanced explanation on features and configuration options, see the [shrs book](mrpicklepinosaur.github.io/shrs/)
//!

#[macro_use]
extern crate derive_builder;

pub use shrs_core::*;
pub use shrs_lang as lang;
pub use shrs_line as line;

mod shell;
pub use shell::*;

pub mod plugin;

pub mod crossterm {
    //! Re-export of crossterm types

    pub use crossterm::{
        style::{Print, Stylize},
        QueueableCommand,
    };
}

pub mod anyhow {
    ///! Re-export of anyhow crate for error handling
    pub use anyhow::{anyhow, Error, Result, *};
}

pub mod prelude {
    //! `use shrs::prelude::*` to import most commonly used structs and functions

    pub use shrs_core::prelude::*;
    pub use shrs_lang::PosixLang;
    pub use shrs_line::{completion::*, *};

    pub use crate::{anyhow, crossterm, crossterm::*, plugin::*, shell::*};
}
