use shrs::{crossterm, prelude::*};

pub struct FloatingMenu {}

impl FloatingMenu {
    pub fn new() -> Self {
        Self {}
    }
}

impl Menu for FloatingMenu {
    type MenuItem = String;
    type PreviewItem = String;

    fn next(&mut self) {
        todo!()
    }

    fn previous(&mut self) {
        todo!()
    }

    fn accept(&mut self) -> Option<&Self::MenuItem> {
        todo!()
    }

    fn current_selection(&self) -> Option<&Self::MenuItem> {
        todo!()
    }

    fn cursor(&self) -> u32 {
        todo!()
    }

    fn is_active(&self) -> bool {
        todo!()
    }

    fn activate(&mut self) {
        todo!()
    }

    fn disactivate(&mut self) {
        todo!()
    }

    fn items(&self) -> Vec<&(Self::PreviewItem, Self::MenuItem)> {
        todo!()
    }

    fn set_items(&mut self, items: Vec<(Self::PreviewItem, Self::MenuItem)>) {
        todo!()
    }

    fn render(&self, out: &mut Out) -> anyhow::Result<()> {
        todo!()
    }

    fn required_lines(&self) -> usize {
        // We don't need to scroll up at all
        0
    }
}
