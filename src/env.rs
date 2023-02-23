use std::collections::HashMap;

/// Environment variables
// currently just wrapper around hashmap
pub struct Env {
    vars: HashMap<String, String>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            vars: HashMap::new(),
        }
    }

    /// Initialize default values for environment variables
    // could inherit all from calling shell for now
    pub fn load(&mut self) {
        for (var, val) in std::env::vars() {
            self.set(&var, &val);
        }
    }

    pub fn get(&self, var: &str) -> Option<&String> {
        self.vars.get(&var.to_ascii_uppercase())
    }

    pub fn set(&mut self, var: &str, val: &str) {
        self.vars.insert(var.to_ascii_uppercase(), val.into());
    }

    pub fn all(&self) -> &HashMap<String, String> {
        &self.vars
    }
}
