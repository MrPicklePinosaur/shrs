use crossterm::style::ContentStyle;
use lazy_static::lazy_static;
use regex::Regex;
use shrs::{
    crossterm::Stylize,
    prelude::{StyledBuf, SyntaxTheme},
};
lazy_static! {
    static ref FILE_PATTERN: Regex = Regex::new(r#"File (".*"), line (\d+), in (.*)"#).unwrap();
    static ref ERROR_PATTERN: Regex = Regex::new(r#"([^:]+:) (.+)"#).unwrap();
}

pub struct PythonErrorTheme {
    pub error_style: ContentStyle,
    pub line_style: ContentStyle,
    pub file_style: ContentStyle,
    pub reason_style: ContentStyle,
}

impl PythonErrorTheme {
    pub fn new() -> Self {
        Self {
            error_style: ContentStyle::new().red(),
            line_style: ContentStyle::new().dark_blue(),
            file_style: ContentStyle::new().green(),
            reason_style: ContentStyle::new().yellow(),
        }
    }
}
impl SyntaxTheme for PythonErrorTheme {
    fn apply(&self, buf: &mut StyledBuf) {
        let content = buf.content.clone();
        if content.starts_with("Traceback") {
            buf.apply_style(self.error_style);
        } else if let Some(captures) = FILE_PATTERN.captures(content.as_str()) {
            if let Some(c) = captures.get(1) {
                buf.apply_style_in_range(c.range(), self.file_style);
            }
            if let Some(c) = captures.get(2) {
                buf.apply_style_in_range(c.range(), self.line_style);
            }
            if let Some(c) = captures.get(3) {
                buf.apply_style_in_range(c.range(), self.line_style);
            }
        } else if let Some(captures) = ERROR_PATTERN.captures(content.as_str()) {
            if let Some(c) = captures.get(1) {
                buf.apply_style_in_range(c.range(), self.error_style);
            }
            if let Some(c) = captures.get(2) {
                buf.apply_style_in_range(c.range(), self.reason_style);
            }
        }
    }
}
