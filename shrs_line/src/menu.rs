use std::fmt::Display;

use crossterm::{
    execute,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
};

pub type Out = std::io::BufWriter<std::io::Stdout>;

pub trait Menu {
    type MenuItem: Display;

    fn next(&mut self);
    fn previous(&mut self);
    fn accept(&mut self) -> Option<&Self::MenuItem>;
    fn cursor(&self) -> u32;
    fn is_active(&self) -> bool;
    fn activate(&mut self);
    fn disactivate(&mut self);
    fn items(&self) -> Vec<&Self::MenuItem>;
    fn set_items(&mut self, items: Vec<Self::MenuItem>);

    fn selected_style(&self, out: &mut Out) -> crossterm::Result<()>;
    fn unselected_style(&self, out: &mut Out) -> crossterm::Result<()>;
}

/// Simple menu that prompts user for a selection
pub struct DefaultMenu {
    selections: Vec<String>,
    /// Currently selected item
    cursor: u32,
    active: bool,
}

impl DefaultMenu {
    pub fn new() -> Self {
        DefaultMenu {
            selections: vec![],
            cursor: 0,
            active: false,
        }
    }
}

impl Menu for DefaultMenu {
    type MenuItem = String;

    fn next(&mut self) {
        self.cursor = if self.selections.is_empty() {
            0
        } else {
            (self.cursor + 1).min(self.selections.len() as u32 - 1)
        };
    }
    fn previous(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
    fn accept(&mut self) -> Option<&String> {
        self.disactivate();
        self.selections.get(self.cursor as usize)
    }
    fn cursor(&self) -> u32 {
        self.cursor
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn activate(&mut self) {
        // dont activate if menu is empty
        self.active = !self.selections.is_empty();
    }
    fn disactivate(&mut self) {
        self.active = false;
    }
    fn items(&self) -> Vec<&String> {
        // TODO is this the right way to case Vec<String> to Vec<&String> ??
        self.selections.iter().collect()
    }
    fn set_items(&mut self, mut items: Vec<Self::MenuItem>) {
        self.selections.clear();
        self.selections.append(&mut items);
        self.cursor = 0;
    }

    fn selected_style(&self, out: &mut Out) -> crossterm::Result<()> {
        execute!(
            out,
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black),
        )?;
        Ok(())
    }

    fn unselected_style(&self, out: &mut Out) -> crossterm::Result<()> {
        execute!(out, ResetColor,)?;
        Ok(())
    }
}
