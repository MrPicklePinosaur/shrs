use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub trait Keybinding {
    /// Return true indicates that event was handled
    fn handle_key_event(&mut self, key_event: KeyEvent) -> bool;
}

pub type Binding = (KeyCode, KeyModifiers);

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
