use std::{fmt::Display, ops::Range};

use crossterm::style::{Attribute, Color, ContentStyle, StyledContent, Stylize};
use unicode_width::UnicodeWidthStr;

/// Text to be rendered by painter
/// styles has a style for each character in content
#[derive(Clone, Debug)]
pub struct StyledBuf {
    pub content: String,
    styles: Vec<ContentStyle>,
}
macro_rules! stylize_buf_method {
    ($method_name:ident Attribute::$attribute:ident) => {
        pub fn $method_name(self) -> StyledBuf {
            self.attribute(Attribute::$attribute)
        }
    };
    ($method_name_fg:ident, $method_name_bg:ident, $method_name_ul:ident Color::$color:ident) => {
        pub fn $method_name_fg(self) -> StyledBuf {
            self.with(Color::$color)
        }

        pub fn $method_name_bg(self) -> StyledBuf {
            self.on(Color::$color)
        }

        pub fn $method_name_ul(self) -> StyledBuf {
            self.underline(Color::$color)
        }
    };
}

impl StyledBuf {
    pub fn empty() -> Self {
        Self {
            content: String::new(),
            styles: vec![],
        }
    }
    pub fn new(content: &str) -> Self {
        let mut s = Self::empty();
        s.push(content, ContentStyle::new());
        s
    }

    pub fn push(&mut self, content: &str, style: ContentStyle) {
        self.content += content;

        for _ in content.chars() {
            self.styles.push(style);
        }
    }

    pub fn lines(&self) -> Vec<Vec<StyledContent<String>>> {
        let mut lines: Vec<Vec<StyledContent<String>>> = vec![];
        let mut i = 0;
        for line in self.content.split('\n') {
            let mut x: Vec<StyledContent<String>> = vec![];

            for c in line.chars() {
                x.push(StyledContent::new(self.styles[i], c.to_string()));
                i += 1;
            }
            i += 1;
            lines.push(x);
        }
        lines
    }
    pub fn spans(&self) -> Vec<StyledContent<String>> {
        let mut x: Vec<StyledContent<String>> = vec![];
        for (i, c) in self.content.chars().enumerate() {
            x.push(StyledContent::new(self.styles[i], c.to_string()));
        }
        x
    }
    //can be simply changed to just the len(lines())-1
    //kept for now
    pub fn count_newlines(&self) -> u16 {
        self.content
            .chars()
            .filter(|c| *c == '\n')
            .count()
            .try_into()
            .unwrap()
    }

    /// Length of content in characters
    ///
    /// The length returned is the 'visual' length of the character, in other words, how many
    /// terminal columns it takes up
    pub fn content_len(&self) -> u16 {
        UnicodeWidthStr::width(self.content.as_str()) as u16
    }

    pub fn apply_style(&mut self, style: ContentStyle) {
        self.styles.iter_mut().for_each(|x| *x = style);
    }

    pub fn apply_style_at(&mut self, index: usize, style: ContentStyle) {
        self.styles[index] = style;
    }
    pub fn apply_style_in_range(&mut self, range: Range<usize>, style: ContentStyle) {
        range.for_each(|u| self.apply_style_at(u, style));
    }
    pub fn slice_from(&self, start: usize) -> StyledBuf {
        if start >= self.content.len() {
            return StyledBuf::empty();
        }

        let sliced_content = &self.content[start..];
        let sliced_styles = self.styles[start..].to_vec();

        StyledBuf {
            content: sliced_content.to_string(),
            styles: sliced_styles,
        }
    }

    pub fn push_buf(&mut self, buf: StyledBuf) {
        self.content += buf.content.as_str();
        self.styles.extend(buf.styles);
    }
    //These methods emulate stylize behaviour
    pub fn with(mut self, color: Color) -> StyledBuf {
        self.styles.iter_mut().for_each(|x| *x = x.with(color));
        self
    }
    pub fn on(mut self, color: Color) -> StyledBuf {
        self.styles.iter_mut().for_each(|x| *x = x.on(color));
        self
    }
    pub fn underline(mut self, color: Color) -> StyledBuf {
        self.styles.iter_mut().for_each(|x| *x = x.underline(color));
        self
    }
    pub fn attribute(mut self, attr: Attribute) -> StyledBuf {
        self.styles.iter_mut().for_each(|x| *x = x.attribute(attr));
        self
    }
    pub fn style(mut self, style: ContentStyle) -> StyledBuf {
        self.styles.iter_mut().for_each(|x| *x = style);
        self
    }

    stylize_buf_method!(reset Attribute::Reset);
    stylize_buf_method!(bold Attribute::Bold);
    stylize_buf_method!(underlined Attribute::Underlined);
    stylize_buf_method!(reverse Attribute::Reverse);
    stylize_buf_method!(dim Attribute::Dim);
    stylize_buf_method!(italic Attribute::Italic);
    stylize_buf_method!(negative Attribute::Reverse);
    stylize_buf_method!(slow_blink Attribute::SlowBlink);
    stylize_buf_method!(rapid_blink Attribute::RapidBlink);
    stylize_buf_method!(hidden Attribute::Hidden);
    stylize_buf_method!(crossed_out Attribute::CrossedOut);

    stylize_buf_method!(black, on_black, underline_black Color::Black);
    stylize_buf_method!(dark_grey, on_dark_grey, underline_dark_grey Color::DarkGrey);
    stylize_buf_method!(red, on_red, underline_red Color::Red);
    stylize_buf_method!(dark_red, on_dark_red, underline_dark_red Color::DarkRed);
    stylize_buf_method!(green, on_green, underline_green Color::Green);
    stylize_buf_method!(dark_green, on_dark_green, underline_dark_green Color::DarkGreen);
    stylize_buf_method!(yellow, on_yellow, underline_yellow Color::Yellow);
    stylize_buf_method!(dark_yellow, on_dark_yellow, underline_dark_yellow Color::DarkYellow);
    stylize_buf_method!(blue, on_blue, underline_blue Color::Blue);
    stylize_buf_method!(dark_blue, on_dark_blue, underline_dark_blue Color::DarkBlue);
    stylize_buf_method!(magenta, on_magenta, underline_magenta Color::Magenta);
    stylize_buf_method!(dark_magenta, on_dark_magenta, underline_dark_magenta Color::DarkMagenta);
    stylize_buf_method!(cyan, on_cyan, underline_cyan Color::Cyan);
    stylize_buf_method!(dark_cyan, on_dark_cyan, underline_dark_cyan Color::DarkCyan);
    stylize_buf_method!(white, on_white, underline_white Color::White);
    stylize_buf_method!(grey, on_grey, underline_grey Color::Grey);
}

pub fn line_content_len(line: Vec<StyledContent<String>>) -> u16 {
    let c = line
        .iter()
        .map(|x| x.content().as_str())
        .collect::<String>();
    UnicodeWidthStr::width(c.as_str()) as u16
}
impl Display for StyledBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)?;
        Ok(())
    }
}
impl FromIterator<StyledBuf> for StyledBuf {
    fn from_iter<T: IntoIterator<Item = StyledBuf>>(iter: T) -> Self {
        let mut buf = Self::empty();
        for i in iter {
            buf.push_buf(i);
        }
        buf
    }
}
impl From<String> for StyledBuf {
    fn from(value: String) -> Self {
        StyledBuf::new(value.as_str())
    }
}
impl From<&str> for StyledBuf {
    fn from(value: &str) -> Self {
        StyledBuf::new(value)
    }
}
impl<T: Display> From<StyledContent<T>> for StyledBuf {
    fn from(value: StyledContent<T>) -> Self {
        StyledBuf::new(value.content().to_string().as_str()).style(*value.style())
    }
}
impl<T: Display> From<Option<StyledContent<T>>> for StyledBuf {
    fn from(value: Option<StyledContent<T>>) -> Self {
        if let Some(v) = value {
            v.into()
        } else {
            StyledBuf::empty()
        }
    }
}
impl<T: Display, E> From<Result<StyledContent<T>, E>> for StyledBuf {
    fn from(value: Result<StyledContent<T>, E>) -> Self {
        if let Ok(v) = value {
            v.into()
        } else {
            StyledBuf::empty()
        }
    }
}
impl From<Option<&str>> for StyledBuf {
    fn from(value: Option<&str>) -> Self {
        value.unwrap_or_default().into()
    }
}
impl<E> From<Result<&str, E>> for StyledBuf {
    fn from(value: Result<&str, E>) -> Self {
        value.unwrap_or_default().into()
    }
}
impl From<Option<String>> for StyledBuf {
    fn from(value: Option<String>) -> Self {
        value.unwrap_or_default().into()
    }
}
impl<E> From<Result<String, E>> for StyledBuf {
    fn from(value: Result<String, E>) -> Self {
        value.unwrap_or_default().into()
    }
}

/// Macro to easily compose [StyledBuf] for use in prompt implementation
///
/// Note need crossterm::style::Stylize
#[macro_export]
macro_rules! styled_buf {
    ($($part:expr),* $(,)*) => {{

        use $crate::StyledBuf;

        StyledBuf::from_iter(vec![
            $(
                $part.into()
            ),*
        ])
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn styled_macro() {
        use crossterm::style::Stylize;
        println!("test {}", "lol".blue().reset());

        let styled_buf = styled_buf! {
            styled_buf!{"lol".blue()},
            Some("lol"),
            "lol",
            String::from("lol"),
            "lol".blue(),
            styled_buf! { "lol" }
        };
        println!("out {styled_buf}");
    }
}
