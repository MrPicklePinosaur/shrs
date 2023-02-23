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
