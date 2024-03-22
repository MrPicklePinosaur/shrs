#![doc(html_root_url = "https://docs.rs/shrs/0.0.4")]

//! **shrs** is a framework for building and configuring your own shell in rust.
//!
//! # Example
//! The most basic shell can be created very easily:
//! ```no_run
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

pub mod lang {
    //! Shell command language

    pub use shrs_lang::*;
}
mod readline;

mod shell;

pub mod plugin;

pub mod crossterm {
    //! Re-export of crossterm types

    pub use crossterm::{
        style::{Attribute, Color, Print, Stylize},
        QueueableCommand,
    };
}

pub mod anyhow {
    //! Re-export of anyhow crate for error handling
    pub use anyhow::{anyhow, Error, Result, *};
}

pub mod prelude {
    //! `use shrs::prelude::*` to import most commonly used structs and functions

    pub use shrs_core::prelude::*;

    pub use shrs_utils::*;

    pub use crate::{anyhow, crossterm, crossterm::*, plugin::*, readline::*, shell::*};
}
