//! Shell aliasing

use std::collections::HashMap;

/*
// NOTE this probably does not need to be a trait since alias is a builtin, any special
// functionality can probaly be implemented in the builtin itself?
pub trait Alias {
    fn get(&self, alias: &str) -> Option<&String>;
    fn set(&mut self, alias: &str, cmd: &str);
    fn unset(&mut self, alias: &str);
    fn clear(&mut self);
}
*/

/// Query and set aliases
///
/// Aliases are stored as the raw string entered, therefore invalid syntax can be set as an alias,
/// but upon substition the error is emitted. This may be changed in the future.
// currently just wrapper around hashmap
#[derive(Clone)]
pub struct Alias {
    aliases: HashMap<String, String>,
}

impl Alias {
    pub fn new() -> Self {
        Alias {
            aliases: HashMap::new(),
        }
    }

    /// Fetch an alias by name
    pub fn get(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }

    /// Set an alias
    ///
    /// Overrides previously defined aliases
    pub fn set(&mut self, alias: &str, cmd: &str) {
        self.aliases.insert(alias.into(), cmd.into());
    }

    /// Remove an alias
    ///
    /// NOOP if alias was not previously defined
    pub fn unset(&mut self, alias: &str) {
        self.aliases.remove(alias);
    }

    /// Remove all defined aliases
    pub fn clear(&mut self) {
        self.aliases.clear();
    }
}

impl FromIterator<(&'static str, &'static str)> for Alias {
    fn from_iter<T: IntoIterator<Item = (&'static str, &'static str)>>(iter: T) -> Self {
        Alias {
            aliases: HashMap::from_iter(
                iter.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())),
            ),
        }
    }
}
