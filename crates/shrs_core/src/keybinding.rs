//! Keybinding system

use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use thiserror::Error;

use crate::shell::{Context, Runtime, Shell};

pub type BindingFn = dyn Fn(&Shell, &mut Context, &mut Runtime);

/// Implement this trait to define your own keybinding system
pub trait Keybinding {
    /// Return true indicates that event was handled
    fn handle_key_event(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        key_event: KeyEvent,
    ) -> bool;
    fn get_info(&self) -> &HashMap<String, String>;
}

pub type Binding = (KeyCode, KeyModifiers);

/// Macro to easily define keybindings
#[macro_export]
macro_rules! keybindings {
    // TODO temp hacky macro
    (|$sh:ident, $ctx:ident, $rt:ident| $($binding:expr => ($desc:expr, $func:block)),* $(,)*) => {{
        use $crate::keybinding::{DefaultKeybinding, parse_keybinding, BindingFn};
        use $crate::prelude::{Shell, Context, Runtime};
        DefaultKeybinding::from_iter([
            $((
                parse_keybinding($binding).unwrap(),
                Box::new(|$sh: &Shell, $ctx: &mut Context, $rt: &mut Runtime| {
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
pub fn parse_keybinding(s: &str) -> Result<Binding, BindingFromStrError> {
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

/// Default implementation of [Keybinding]
#[derive(Default)]
pub struct DefaultKeybinding {
    // TODO this can't take closure right now
    pub bindings: HashMap<Binding, (Box<BindingFn>)>,
    pub info: HashMap<String, String>,
}

impl DefaultKeybinding {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            info: HashMap::new(),
        }
    }
}

impl Keybinding for DefaultKeybinding {
    fn handle_key_event(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        key_event: KeyEvent,
    ) -> bool {
        let mut event_handled = false;
        for (binding, binding_fn) in self.bindings.iter() {
            if (key_event.code, key_event.modifiers) == *binding {
                binding_fn(sh, ctx, rt);
                event_handled = true;
            }
        }
        event_handled
    }

    fn get_info(&self) -> &HashMap<String, String> {
        &self.info
    }
}

impl FromIterator<(Binding, Box<BindingFn>, String, String)> for DefaultKeybinding {
    fn from_iter<T: IntoIterator<Item = (Binding, Box<BindingFn>, String, String)>>(
        iter: T,
    ) -> Self {
        let mut default_keybinding = DefaultKeybinding {
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
