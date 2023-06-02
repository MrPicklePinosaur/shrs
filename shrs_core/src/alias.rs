//! Shell aliasing

use std::collections::HashMap;

use multimap::MultiMap;

pub struct AliasRuleCtx {}
pub struct AliasRule(Box<dyn Fn(&AliasRuleCtx) -> bool>);

pub struct AliasInfo {
    /// The actual value to be substituted
    pub subst: String,
    /// Predicate to decide if the alias should be taken or not
    pub rule: AliasRule,
}

impl AliasInfo {
    /// Always use this alias
    pub fn always(subst: String) -> Self {
        Self {
            subst,
            rule: AliasRule(Box::new(|ctx| -> bool { true })),
        }
    }
}

/// Query and set aliases
///
/// Aliases are stored as the raw string entered, therefore invalid syntax can be set as an alias,
/// but upon substition the error is emitted. This may be changed in the future.
pub struct Alias {
    aliases: MultiMap<String, AliasInfo>,
}

impl Alias {
    pub fn new() -> Self {
        Alias {
            aliases: MultiMap::new(),
        }
    }

    /// Fetch an alias by name while respecting alias rules
    pub fn get(&self, alias: &str) -> Option<&String> {
        self.aliases
            .get_vec(alias)
            .unwrap()
            .iter()
            .filter(|alias_info| {
                // alias_info.rule()
                todo!()
            });
        todo!()
    }

    /// Set an alias
    ///
    /// Overrides previously defined aliases
    pub fn set(&mut self, alias: &str, cmd: &str) {
        // self.aliases.insert(alias.into(), cmd.into());
        todo!()
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

impl FromIterator<(String, String)> for Alias {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        // Alias {
        //     aliases: HashMap::from_iter(
        //         iter.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())),
        //     ),
        // }
        todo!()
    }
}

impl FromIterator<(&'static str, &'static str)> for Alias {
    fn from_iter<T: IntoIterator<Item = (&'static str, &'static str)>>(iter: T) -> Self {
        // Alias {
        //     aliases: HashMap::from_iter(
        //         iter.into_iter().map(|(k, v)| (k.to_owned(), v.to_owned())),
        //     ),
        // }
        todo!()
    }
}
