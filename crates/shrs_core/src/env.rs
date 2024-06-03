//! Environment variables
//!
//! You can load all the current environment variables into shrs by using [`Env::load`]. This is
//! useful in the case that you are launching your shrs shell from another shell, like bash.
//! ```
//! # use shrs_core::prelude::*;
//! # let mut myshell = ShellBuilder::default();
//! let mut env = Env::new();
//! env.load();
//!
//! myshell.with_env(env);
//! ```
//!
//! In the case that the shrs shell is your login shell, or that you wish to define additional
//! environment variables, you can do so by appending to the [`Env`] object. Note that environment
//! variables are also case insensitive.
//! ```ignore
//! env.set("SHELL", "my_shrs");
//! ```

use std::{collections::HashMap, env};

use thiserror::Error;

/// Hook for when environment variable gets modified
pub struct EnvModifiedCtx {
    /// Name of the environment variable
    pub var: String,
    /// Current value of the variable
    ///
    /// Value of [None] means that the variable was unset
    pub new_val: Option<String>,
    /// Old value of the variable
    ///
    /// Value of [None] means that the variable was not previously set
    pub old_val: Option<String>,
}

// TODO run hooks
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct Env {
    var_table: HashMap<String, String>,
}

impl Env {
    /// Create a new environment variable holder
    pub fn new() -> Self {
        Env {
            var_table: HashMap::new(),
        }
    }

    /// Load environment variables into shrs
    ///
    /// Useful if calling shrs from another shell and some environment variables are already set
    // could inherit all from calling shell for now
    pub fn load(&mut self) -> Result<(), EnvError> {
        for (var, val) in std::env::vars() {
            self.set(&var, &val)?;
        }
        Ok(())
    }

    /// Query environment variable
    pub fn get(&self, var: &str) -> Result<&String, EnvError> {
        // env::var(var).map_err(|_| EnvError::NotFound(var.into()))
        self.var_table
            .get(var)
            .ok_or_else(|| EnvError::NotFound(var.into()))
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

        env::set_var(var, val);
        self.var_table.insert(var.into(), val.into());
        Ok(())
    }

    /// Obtain an iterator of all the environment variables
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        // env::vars_os()
        self.var_table.iter()
    }

    /// Unset an environment variable
    ///
    /// If the environment variable was already not set, it is a NOOP
    pub fn remove(&mut self, var: &str) -> Result<(), EnvError> {
        if key_sanitation(var) {
            return Err(EnvError::InvalidKey(var.into()));
        }
        env::remove_var(var);
        self.var_table.remove(var);
        Ok(())
    }

    /// Writes all of the currently defined environment variables to the process
    pub fn sync(&self) -> Result<(), EnvError> {
        unimplemented!()
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

impl<S: ToString> FromIterator<(S, S)> for Env {
    fn from_iter<T: IntoIterator<Item = (S, S)>>(iter: T) -> Self {
        Env {
            var_table: HashMap::from_iter(
                iter.into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string())),
            ),
        }
    }
}
