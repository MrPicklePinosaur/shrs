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

    fn get(&self, var: &str) -> Option<&String> {
        self.vars.get(var)
    }

    fn set(&mut self, var: &str, val: &str) {
        self.vars.insert(var.into(), val.into());
    }
}
