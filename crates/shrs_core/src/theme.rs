//! Configuration for common color values bundled into a theme

use crossterm::style::Color;

pub struct Theme {
    pub out_color: Color,
    pub err_color: Color,
    pub selection_color: Color,
    pub black: Color,
    pub dark_grey: Color,
    pub red: Color,
    pub dark_red: Color,
    pub green: Color,
    pub dark_green: Color,
    pub yellow: Color,
    pub dark_yellow: Color,
    pub blue: Color,
    pub dark_blue: Color,
    pub magenta: Color,
    pub dark_magenta: Color,
    pub cyan: Color,
    pub dark_cyan: Color,
    pub white: Color,
    pub light_grey: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            out_color: Color::White,
            err_color: Color::Red,
            selection_color: Color::White,
            black: Color::Black,
            dark_grey: Color::DarkGrey,
            red: Color::Red,
            dark_red: Color::DarkRed,
            green: Color::Green,
            dark_green: Color::DarkGreen,
            yellow: Color::Yellow,
            dark_yellow: Color::DarkYellow,
            blue: Color::Blue,
            dark_blue: Color::DarkBlue,
            magenta: Color::Magenta,
            dark_magenta: Color::DarkMagenta,
            cyan: Color::Cyan,
            dark_cyan: Color::DarkCyan,
            white: Color::White,
            light_grey: Color::Grey,
        }
    }
}
