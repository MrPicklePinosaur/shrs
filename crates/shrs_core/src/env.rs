//! Environment variables

use std::{collections::HashMap, env, ffi::OsString};

use shrs_utils::warn_if_err;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("Malformed key: {0}")]
    InvalidKey(String),
    #[error("Malformed value: {0}")]
    InvalidValue(String),
    #[error("Key not found: {0}")]
    NotFound(String),
}

/// Set and query environment variables
///
/// This is just a wrapper around the functions from std::env
#[derive(Clone)]
pub struct Env {}

impl Env {
    /// Construct a new [Env] struct
    pub fn new() -> Self {
        Env {}
    }

    /// Load environment variables into shrs
    ///
    /// Useful if calling shrs from another shell and some environment variables are already set
    // could inherit all from calling shell for now
    pub fn load(&mut self) {
        // NO-OP
        // for (var, val) in std::env::vars() {
        //     self.set(&var, &val);
        // }
    }

    /// Query environment variable
    pub fn get(&self, var: &str) -> Result<String, EnvError> {
        env::var(var).map_err(|_| EnvError::NotFound(var.into()))
    }

    /// Set an environment variable
    ///
    /// If the variable was already set it is overridden. Environment variables are case
    /// insensitive
    pub fn set(&mut self, var: &str, val: &str) -> Result<(), EnvError> {
        // Careful: env::set_var will panic in the following cases (from the docs)
        //
        // This function may panic if key is empty, contains an ASCII equals sign '=' or the NUL
        // character '\0', or when value contains the NUL character.
        //
        if key_sanitation(var) {
            return Err(EnvError::InvalidKey(var.into()));
        }

        if val_sanitation(val) {
            return Err(EnvError::InvalidValue(val.into()));
        }

        env::set_var(var.to_ascii_uppercase(), val);
        Ok(())
    }

    /// Obtain an interator of all the environment variables
    pub fn all(&self) -> impl Iterator<Item = (OsString, OsString)> {
        env::vars_os()
    }

    /// Unset an environment variable
    ///
    /// If the environment variable was already not set, it is a NOOP
    pub fn remove(&mut self, var: &str) -> Result<(), EnvError> {
        if key_sanitation(var) {
            return Err(EnvError::InvalidKey(var.into()));
        }
        env::remove_var(var);
        Ok(())
    }
}

/// Checks if a environment variable name is valid
fn key_sanitation(var: &str) -> bool {
    var.is_empty() || var.contains('=') || var.contains('\0')
}

/// Checks if a environment variable value is valid
fn val_sanitation(val: &str) -> bool {
    val.contains('\0')
}

impl FromIterator<(&'static str, &'static str)> for Env {
    fn from_iter<T: IntoIterator<Item = (&'static str, &'static str)>>(iter: T) -> Self {
        // Env {
        //     vars: HashMap::from_iter(iter.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned()))),
        // }
        todo!()
    }
}
