use std::collections::HashMap;

// currently just wrapper around hashmap
pub struct Alias {
    aliases: HashMap<String, String>,
}

impl Alias {
    pub fn new() -> Self {
        Alias {
            aliases: HashMap::new(),
        }
    }

    pub fn get(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }

    pub fn set(&mut self, alias: &str, cmd: &str) {
        self.aliases.insert(alias.into(), cmd.into());
    }

    pub fn unset(&mut self, alias: &str) {
        self.aliases.remove(alias);
    }
}
