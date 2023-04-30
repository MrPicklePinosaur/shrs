pub trait Keybinding {}

pub struct DefaultKeybinding {}

impl DefaultKeybinding {
    pub fn new() -> Self {
        Self {}
    }
}

impl Keybinding for DefaultKeybinding {}
