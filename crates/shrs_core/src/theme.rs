//! Configuration for common color values bundled into a theme

use crossterm::style::Color;

pub struct Theme {
    pub out_color: Color,
    pub err_color: Color,
    pub selection_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            out_color: Color::White,
            err_color: Color::Red,
            selection_color: Color::White,
        }
    }
}
