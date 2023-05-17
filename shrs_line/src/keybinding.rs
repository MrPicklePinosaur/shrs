use std::{collections::HashMap, error::Error, str::FromStr};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use thiserror::Error;

pub trait Keybinding {
    /// Return true indicates that event was handled
    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool;
}

// #[macro_export]
// macro_rules! keybindings {
//     ($($binding:expr),* $(,)*) => {{

//     }};
// }

pub type Binding = (KeyCode, KeyModifiers);

#[derive(Error, Debug)]
enum BindingFromStrError {
    #[error("unknown key: {0}")]
    UnknownKey(String),
    #[error("unknown modifier: {0}")]
    UnknownMod(String),
    #[error("empty keybinding")]
    EmptyKeybinding,
}

fn parse_keybinding(s: &str) -> Result<Binding, BindingFromStrError> {
    let mut parts = s.split("-").collect::<Vec<_>>();

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
            if '!' <= c && c <= '~' {
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
        _ => Err(BindingFromStrError::UnknownMod(s.to_string())),
    }
}

pub struct DefaultKeybinding {
    // TODO this can't take closure right now
    pub bindings: HashMap<Binding, Box<dyn FnMut()>>,
}

impl DefaultKeybinding {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}

impl Keybinding for DefaultKeybinding {
    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool {
        let mut event_handled = false;
        for (binding, binding_fn) in self.bindings.iter_mut() {
            if (key_event.code, key_event.modifiers) == *binding {
                binding_fn();
                event_handled = true;
            }
        }
        event_handled
    }
}

impl FromIterator<(Binding, Box<dyn FnMut()>)> for DefaultKeybinding {
    fn from_iter<T: IntoIterator<Item = (Binding, Box<dyn FnMut()>)>>(iter: T) -> Self {
        DefaultKeybinding {
            bindings: HashMap::from_iter(iter),
        }
    }
}
