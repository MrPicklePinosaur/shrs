//! Shell aliasing
//!
//! ```
//! # use shrs_core::prelude::*;
//! let alias = Alias::from_iter([
//!     ("l", "ls"),
//!     ("c", "cd"),
//!     ("g", "git"),
//!     ("v", "vim"),
//!     ("la", "ls -a"),
//! ]);
//! ```

use multimap::MultiMap;

use crate::shell::{Context, Runtime, Shell};

/// Parameters passed to alias rule
pub struct AliasRuleCtx<'a> {
    pub alias_name: &'a str,
    pub sh: &'a Shell,
    pub ctx: &'a Context,
    pub rt: &'a Runtime,
}
/// Predicate to decide if an alias should be used or not
pub struct AliasRule(Box<dyn Fn(&AliasRuleCtx) -> bool>);

/// Contains alias value and other metadata
pub struct AliasInfo {
    /// The actual value to be substituted
    pub subst: String,
    /// Predicate to decide if the alias should be taken or not
    pub rule: AliasRule,
}

impl AliasInfo {
    /// Always use this alias
    pub fn always<S: ToString>(subst: S) -> Self {
        Self {
            subst: subst.to_string(),
            rule: AliasRule(Box::new(|_| -> bool { true })),
        }
    }

    /// Conditionally run this alias
    pub fn with_rule<S, R>(subst: S, rule: R) -> Self
    where
        S: ToString,
        R: Fn(&AliasRuleCtx) -> bool + 'static,
    {
        Self {
            subst: subst.to_string(),
            rule: AliasRule(Box::new(rule)),
        }
    }
}

/// Query and set aliases
///
/// Aliases are stored as the raw string entered, therefore invalid syntax can be set as an alias,
/// but upon substitution the error is emitted. This may be changed in the future.
#[derive(Default)]
pub struct Alias {
    aliases: MultiMap<String, AliasInfo>,
}

impl Alias {
    /// Fetch all possible aliases
    pub fn get(&self, alias_ctx: &AliasRuleCtx) -> Vec<&String> {
        let alias_list = match self.aliases.get_vec(alias_ctx.alias_name) {
            Some(alias_list) => alias_list,
            None => return vec![],
        };

        alias_list
            .iter()
            .filter(|alias_info| (alias_info.rule.0)(alias_ctx))
            .map(|alias_info| &alias_info.subst)
            .collect::<Vec<_>>()
    }

    /// Set an alias
    pub fn set(&mut self, alias_name: &str, alias_info: AliasInfo) {
        self.aliases.insert(alias_name.into(), alias_info);
    }

    /// Clear an aliass
    ///
    /// This removes ALL aliases of a given name
    pub fn unset(&mut self, alias_name: &str) {
        self.aliases.remove(alias_name);
    }

    /// Remove all defined aliases
    pub fn clear(&mut self) {
        self.aliases.clear();
    }
}

/// Construct an alias from iterator
///
/// Currently it is not possible to insert rules using FromIterator method. If you wish to add a
/// conditional alias, please insert directly it using the [set()] method
impl<S: ToString> FromIterator<(S, S)> for Alias {
    fn from_iter<T: IntoIterator<Item = (S, S)>>(iter: T) -> Self {
        let iter = iter
            .into_iter()
            .map(|(k, v)| (k.to_string(), AliasInfo::always(v)));
        Alias {
            aliases: MultiMap::from_iter(iter),
        }
    }
}
