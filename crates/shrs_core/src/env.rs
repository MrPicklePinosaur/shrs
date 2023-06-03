//! Environment variables

use std::collections::HashMap;

/// Set and query environment variables
#[derive(Clone)]
pub struct Env {
    vars: HashMap<String, String>,
}

impl Env {
    /// Construct a new [Env] struct
    pub fn new() -> Self {
        Env {
            vars: HashMap::new(),
        }
    }

    /// Load environment variables into shrs
    ///
    /// Useful if calling shrs from another shell and some environment variables are already set
    // could inherit all from calling shell for now
    pub fn load(&mut self) {
        for (var, val) in std::env::vars() {
            self.set(&var, &val);
        }
    }

    /// Query environment variable
    pub fn get(&self, var: &str) -> Option<&String> {
        self.vars.get(&var.to_ascii_uppercase())
    }

    /// Set an environment variable
    ///
    /// If the variable was already set it is overridden. Environment variables are case
    /// insensitive
    pub fn set(&mut self, var: &str, val: &str) {
        self.vars.insert(var.to_ascii_uppercase(), val.into());
    }

    /// Obtain a hashmap of all the environment variables
    pub fn all(&self) -> &HashMap<String, String> {
        &self.vars
    }

    /// Unset an environment variable
    ///
    /// If the environment variable was already not set, it is a NOOP
    pub fn remove(&mut self, var: &str) {
        self.vars.remove(var);
    }
}

impl FromIterator<(&'static str, &'static str)> for Env {
    fn from_iter<T: IntoIterator<Item = (&'static str, &'static str)>>(iter: T) -> Self {
        Env {
            vars: HashMap::from_iter(iter.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned()))),
        }
    }
}
