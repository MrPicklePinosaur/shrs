//! Variety of utilities for running commands conditionally on directory change
//!
//!
use std::{io::BufWriter, marker::PhantomData};

pub mod git;
pub mod rust;
