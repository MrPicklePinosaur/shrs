//! Command aliases
//!
//! Aliases can be specified as a key value pair of the alias name and the actual command it expands to. Keep in mind that aliases are not evaluated or syntax checked at time of definition, only during substitution. This means that it is possible to define aliases that are invalid commands.
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
//! It is also possible to set one alias at a time using [`Alias::set()`], which let's you employ more
//! complex control flow when setting aliases. This is the equivalent of the above:
//! ```
//! # use shrs_core::prelude::*;
//! let mut alias = Alias::new();
//! alias.set("l", AliasInfo::always("ls"));
//! alias.set("c", AliasInfo::always("cd"));
//! alias.set("g", AliasInfo::always("git"));
//! alias.set("v", AliasInfo::always("vim"));
//! alias.set("la", AliasInfo::always("ls -a"));
//! ```
//! You have have noticed the usage of [`AliasInfo::always`], which will unconditionally expand the
//! alias. [`AliasInfo`] also features the ability to conditionally execute aliases based on a
//! predicate. This allows you to enable/disable groups of aliases at runtime, for example only
//! enable git aliases when in a git repo. It is not yet supported to add conditional aliases using
//! [`Alias::from_iter()`], so you must use the [`Alias::set()`] syntax. The below shows how you
//! can make your ls rainbow only on Fridays:
//! ```
//! use chrono::{Datelike, Local, Weekday};
//!
//! let mut alias = Alias::new();
//! let ls_alias = AliasInfo::with_rule("ls | lolcat", |ctx: &AliasRuleCtx| -> bool {
//!     let weekday = Local::now().weekday();
//!     weekday == Weekday::Fri
//! });
//! alias.set("ls", ls_alias);
//! ```

use multimap::MultiMap;

use crate::{prelude::States, shell::Shell};

/// Parameters passed to alias rule
pub struct AliasRuleCtx<'a> {
    /// Name of the aliases that was ran
    pub alias_name: &'a str,
    pub sh: &'a Shell,
    pub states: &'a States,
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
pub struct Alias {
    aliases: MultiMap<String, AliasInfo>,
}

impl Alias {
    /// Construct a new alias map
    pub fn new() -> Self {
        Alias {
            aliases: MultiMap::new(),
        }
    }

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

    /// For a given alias, check what it will evaluate to
    pub fn get_subst(&self, alias_name: &String) -> Option<&String> {
        match self.aliases.get(alias_name) {
            Some(alias_info) => Some(&alias_info.subst),
            None => None,
        }
    }

    /// Update an alias of given name
    pub fn set(&mut self, alias_name: &str, alias_info: AliasInfo) {
        self.aliases.insert(alias_name.into(), alias_info);
    }

    /// Clear an alias
    ///
    /// This removes ALL aliases of a given name.
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
