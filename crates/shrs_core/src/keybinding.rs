//! Keybinding system
//!
//! Keybindings allow you to register certain sequences of keys, which upon entry will trigger a
//! custom defined keybinding handler. This allows you to define ergonomic shortcuts for a variety
//! of actions.
//!
//! Let's see an example of how we would map the keys `Ctrl+L` to run a command to clear the
//! screen. The call to [`Keybindings::insert`], takes in a keybinding string (see [`parse_keybinding`] for what is a valid keybinding string), a description, and a keybinding handler function.
//! ```
//! # use shrs_core::prelude::*;
//! let mut bindings = Keybindings::new();
//!
//! bindings
//!     .insert("C-l", "Clear the screen", || -> anyhow::Result<()> {
//!         std::process::Command::new("clear")
//!             .spawn()
//!             .expect("Couldn't clear screen");
//!         Ok(())
//!     })
//!     .unwrap();
//!
//! let myshell = ShellBuilder::default().with_keybindings(bindings);
//! ```

use std::{collections::HashMap, marker::PhantomData};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use thiserror::Error;

use super::state::Param;
use crate::{
    all_the_tuples,
    prelude::{Shell, States},
};

/// Shell state containing registered keybindings
pub struct Keybindings {
    bindings: HashMap<KeyEvent, Box<dyn Keybinding>>,
    info: HashMap<String, String>,
}

impl Keybindings {
    /// Initialize the Keybindings state struct
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            info: HashMap::new(),
        }
    }

    /// Insert a new keybinding and handler
    ///
    /// Takes in a keybinding string, which will be parsed by [`parse_keybinding`], a description
    /// for the keybinding, and a keybinding handler function. Multiple keybinding handlers for
    /// the same keybinding can be registered, and all will be evaluated
    pub fn insert<I, K: Keybinding + 'static>(
        &mut self,
        key: &str,
        info: &str,
        binding: impl IntoKeybinding<I, Keybinding = K>,
    ) -> Result<()> {
        let b = Box::new(binding.into_keybinding());
        let key_event = parse_keybinding(key)?;
        self.bindings.insert(key_event, b);
        self.info.insert(key.to_string(), info.to_string());
        Ok(())
    }

    /// Attempt to evaluate any registered keybindings
    ///
    /// Return true indicates that some event was handled.
    pub fn handle_key_event(&self, sh: &Shell, states: &States, key_event: KeyEvent) -> bool {
        let mut handled = false;
        for (k, v) in self.bindings.iter() {
            if key_event == *k {
                v.run(sh, states).unwrap();
                handled = true;
            }
        }
        return handled;
    }

    /// Get all the keybindings and their respective descriptions
    // TODO, would be nice to return an iterator
    pub fn get_info(&self) -> &HashMap<String, String> {
        &self.info
    }
}

/// Errors from parsing keybinding from string
#[derive(Error, Debug, PartialEq, Eq)]
pub enum BindingFromStrError {
    #[error("unknown key: {0}")]
    UnknownKey(String),
    #[error("unknown modifier: {0}")]
    UnknownMod(String),
    #[error("empty keybinding")]
    EmptyKeybinding,
}

/// Parse a keybinding from a keybinding string
///
/// This function allows us to represent key combinations in terms of strings. You can also include modifier keys (such as control and shift). The supported modifiers are
///
/// |Modifier|Usage|
/// |---|---|
/// |Shift|"S" or "Shift"|
/// |Alt|"A" or "Alt"|
/// |Control|"C" or "Ctrl"|
/// |Super|"Super"|
/// |Meta|"M" or "Meta"|
///
/// Furthermore, there are also some keys that are hard to represent in a string, so we use a special string to denote them
///
/// |Key|Usage|
/// |---|---|
/// |Space|"\<space>"|
/// |Backspace|"\<backspace>"|
/// |Escape|"\<esc>"|
/// |Enter|"\<enter>"|
/// |Tab|"\<tab>"|
///
/// Some example keybinding strings include
///
/// |Meaning|Usage|
/// |---|---|
/// |Control + Alt + q|"C-A-q"|
/// |Super + Space|"Super-\<space>"|
/// |Alt + Tab|"A-\<tab>"|
///
pub fn parse_keybinding(s: &str) -> Result<KeyEvent, BindingFromStrError> {
    let mut parts = s.split('-').collect::<Vec<_>>();

    // last part is always the keycode
    let keycode_str = parts.pop().ok_or(BindingFromStrError::EmptyKeybinding)?;
    let keycode = parse_keycode(keycode_str)?;

    // parse any leading keycodes
    let mut mods = KeyModifiers::NONE;
    for part in parts {
        let modifier = parse_modifier(part)?;
        mods.set(modifier, true);
    }

    Ok(KeyEvent::new(keycode, mods))
}

/// Parse the keycode part of keybinding
fn parse_keycode(s: &str) -> Result<KeyCode, BindingFromStrError> {
    if s.len() == 1 {
        if let Some(c) = s.chars().next() {
            if ('!'..='~').contains(&c) {
                return Ok(KeyCode::Char(c));
            }
        }
    }

    match s {
        "<space>" => Ok(KeyCode::Char(' ')),
        "<backspace>" => Ok(KeyCode::Backspace),
        "<delete>" => Ok(KeyCode::Delete),
        "<down>" => Ok(KeyCode::Down),
        "<esc>" => Ok(KeyCode::Esc),
        "<enter>" => Ok(KeyCode::Enter),
        "<left>" => Ok(KeyCode::Left),
        "<right>" => Ok(KeyCode::Right),
        "<tab>" => Ok(KeyCode::Tab),
        "<up>" => Ok(KeyCode::Up),
        _ => Err(BindingFromStrError::UnknownKey(s.to_string())),
    }
}

/// Parse the modifier part of keybinding
fn parse_modifier(s: &str) -> Result<KeyModifiers, BindingFromStrError> {
    match s.to_ascii_lowercase().as_str() {
        "s" | "shift" => Ok(KeyModifiers::SHIFT),
        "a" | "alt" => Ok(KeyModifiers::ALT),
        "c" | "ctrl" => Ok(KeyModifiers::CONTROL),
        "super" => Ok(KeyModifiers::SUPER),
        "m" | "meta" => Ok(KeyModifiers::META),
        _ => Err(BindingFromStrError::UnknownMod(s.to_string())),
    }
}

#[cfg(test)]
mod tests {

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::parse_keybinding;

    #[test]
    fn keybinding_parse() {
        assert_eq!(
            parse_keybinding("<space>"),
            Ok(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE))
        );
        assert_eq!(
            parse_keybinding("<esc>"),
            Ok(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE))
        );
        assert_eq!(
            parse_keybinding("c"),
            Ok(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE))
        );
        assert_eq!(
            parse_keybinding("C"),
            Ok(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::NONE))
        );
        assert_eq!(
            parse_keybinding("C-c"),
            Ok(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        );
        assert_eq!(
            parse_keybinding("Ctrl-c"),
            Ok(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL))
        );
        assert_eq!(
            parse_keybinding("C-S-c"),
            Ok(KeyEvent::new(
                KeyCode::Char('c'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT
            ))
        );
        assert_eq!(
            parse_keybinding("Ctrl-Shift-c"),
            Ok(KeyEvent::new(
                KeyCode::Char('c'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT
            ))
        );
    }

    // #[test]
    // fn keybinding_macro() {
    //     keybindings! {
    //         "C-l" => Command::new("clear").spawn(),
    //         "C-q" => Command::new("clear").spawn(),
    //     };
    // }
}

/// Keybinding handler function
pub trait Keybinding {
    fn run(&self, sh: &Shell, states: &States) -> Result<()>;
}

pub trait IntoKeybinding<Input> {
    type Keybinding: Keybinding;
    fn into_keybinding(self) -> Self::Keybinding;
}

pub struct FunctionKeybinding<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

macro_rules! impl_keybinding {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, $($params: Param),*> Keybinding for FunctionKeybinding<($($params,)*), F>
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),*)->Result<()> +
                    Fn( $(<$params as Param>::Item<'b>),* )->Result<()>
        {
            fn run(&self, sh: &Shell,states: &States,  )->Result<()> {
                fn call_inner<$($params),*>(
                    f: impl Fn($($params),*)->Result<()>,
                    $($params: $params),*
                ) -> Result<()>{
                    f($($params),*)
                }

                $(
                    let $params = $params::retrieve(sh,states).unwrap();
                )*

                call_inner(&self.f, $($params),*)
            }
        }
    }
}

macro_rules! impl_into_keybinding {
    (
        $($params:ident),*
    ) => {
        impl<F, $($params: Param),*> IntoKeybinding<($($params,)*)> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),* ) ->Result<()>+
                    Fn( $(<$params as Param>::Item<'b>),* )->Result<()>
        {
            type Keybinding = FunctionKeybinding<($($params,)*), Self>;

            fn into_keybinding(self) -> Self::Keybinding {
                FunctionKeybinding {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}

impl_keybinding!();
impl_into_keybinding!();
all_the_tuples!(impl_keybinding, impl_into_keybinding);
