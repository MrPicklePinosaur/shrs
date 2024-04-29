//! Keybinding system

use super::state::Param;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{collections::HashMap, marker::PhantomData};
use thiserror::Error;

use crate::prelude::{Shell, States};

/// Implement this trait to define your own keybinding system
pub struct Keybindings {
    pub bindings: HashMap<KeyEvent, Box<dyn Keybinding>>,
    pub info: HashMap<String, String>,
}
impl Keybindings {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            info: HashMap::new(),
        }
    }
    fn insert<I, K: Keybinding + 'static>(
        &mut self,
        key: KeyEvent,
        binding: impl IntoKeybinding<I, Keybinding = K>,
    ) {
        let b = Box::new(binding.into_keybinding());
        self.bindings.insert(key, b);
    }
    /// Return true indicates that event was handled
    fn handle_key_event(&self, sh: &Shell, states: &States, key_event: KeyEvent) -> bool {
        for (k, v) in self.bindings.iter() {
            if key_event == *k {
                v.run(sh, states);
                return true;
            }
        }
        return false;
    }

    fn get_info(&self) -> &HashMap<String, String> {
        &self.info
    }
}
impl FromIterator<(KeyEvent, Box<BindingFn>, String, String)> for Keybindings {
    fn from_iter<T: IntoIterator<Item = (Binding, Box<BindingFn>, String, String)>>(
        iter: T,
    ) -> Self {
        let mut default_keybinding = Keybindings {
            bindings: HashMap::new(),
            info: HashMap::new(),
        };
        for item in iter {
            default_keybinding.bindings.insert(item.0, item.1);
            default_keybinding.info.insert(item.2, item.3);
        }
        default_keybinding
    }
}

/// Macro to easily define keybindings
#[macro_export]
macro_rules! keybindings {
    // TODO temp hacky macro

    (|$state:ident| $($binding:expr => ($desc:expr, $func:block)),* $(,)*) => {{
        use $crate::keybinding::{DefaultKeybinding, parse_keybinding, BindingFn};
        use $crate::prelude::{LineStateBundle};
        #[allow(unused)]
        DefaultKeybinding::from_iter([
            $((
                parse_keybinding($binding).unwrap(),
                Box::new(|$state: &mut LineStateBundle| {
                    $func;
                }) as Box<BindingFn>,
                $binding.to_string(),
                $desc.to_string(),
            )),*
        ])
    }};
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

    Ok((keycode, mods))
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
    use crossterm::event::{KeyCode, KeyModifiers};

    use super::parse_keybinding;

    #[test]
    fn keybinding_parse() {
        assert_eq!(
            parse_keybinding("<space>"),
            Ok((KeyCode::Char(' '), KeyModifiers::NONE))
        );
        assert_eq!(
            parse_keybinding("<esc>"),
            Ok((KeyCode::Esc, KeyModifiers::NONE))
        );
        assert_eq!(
            parse_keybinding("c"),
            Ok((KeyCode::Char('c'), KeyModifiers::NONE))
        );
        assert_eq!(
            parse_keybinding("C"),
            Ok((KeyCode::Char('C'), KeyModifiers::NONE))
        );
        assert_eq!(
            parse_keybinding("C-c"),
            Ok((KeyCode::Char('c'), KeyModifiers::CONTROL))
        );
        assert_eq!(
            parse_keybinding("Ctrl-c"),
            Ok((KeyCode::Char('c'), KeyModifiers::CONTROL))
        );
        assert_eq!(
            parse_keybinding("C-S-c"),
            Ok((
                KeyCode::Char('c'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT
            ))
        );
        assert_eq!(
            parse_keybinding("Ctrl-Shift-c"),
            Ok((
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

/// Implement this trait to define your own keybinding command
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

impl<F> Keybinding for FunctionKeybinding<Shell, F>
where
    for<'a, 'b> &'a F: Fn(&Shell) -> Result<()>,
{
    fn run(&self, sh: &Shell, ctx: &States) -> Result<()> {
        fn call_inner(f: impl Fn(&Shell) -> Result<()>, sh: &Shell) -> Result<()> {
            f(&sh)
        }

        call_inner(&self.f, sh)
    }
}

macro_rules! impl_keybinding {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, $($params: Param),+> Keybinding for FunctionKeybinding<($($params,)+), F>
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell,)->Result<()> +
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell, )->Result<()>
        {
            fn run(&self, sh: &Shell,states: &States,  )->Result<()> {
                fn call_inner<$($params),+>(
                    f: impl Fn($($params),+,&Shell,)->Result<()>,
                    $($params: $params),*
                    ,sh:&Shell,
                ) -> Result<()>{
                    f($($params),*,sh)
                }

                $(
                    let $params = $params::retrieve(states);
                )+

                call_inner(&self.f, $($params),+,sh,)
            }
        }
    }
}

impl<F> IntoKeybinding<()> for F
where
    for<'a, 'b> &'a F: Fn(&Shell) -> Result<()>,
{
    type Keybinding = FunctionKeybinding<Shell, Self>;

    fn into_keybinding(self) -> Self::Keybinding {
        FunctionKeybinding {
            f: self,
            marker: Default::default(),
        }
    }
}

macro_rules! impl_into_keybinding {
    (
        $($params:ident),+
    ) => {
        impl<F, $($params: Param),+> IntoKeybinding<($($params,)*)> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell, ) ->Result<()>+
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell, )->Result<()>
        {
            type Keybinding = FunctionKeybinding<($($params,)+), Self>;

            fn into_keybinding(self) -> Self::Keybinding {
                FunctionKeybinding {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}

impl_keybinding!(T1);
impl_keybinding!(T1, T2);
impl_keybinding!(T1, T2, T3);
impl_keybinding!(T1, T2, T3, T4);
impl_into_keybinding!(T1);
impl_into_keybinding!(T1, T2);
impl_into_keybinding!(T1, T2, T3);
impl_into_keybinding!(T1, T2, T3, T4);
