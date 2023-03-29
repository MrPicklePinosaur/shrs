use std::collections::HashMap;

use crossterm::style::{ContentStyle, StyledContent};

use crate::painter::StyledBuf;

pub trait Highlighter {
    fn highlight(&self, buf: &str) -> StyledBuf;
}

/// Simple highlighter that colors the entire line one color
pub struct DefaultHighlighter {
    pub style: ContentStyle,
}

impl Default for DefaultHighlighter {
    fn default() -> Self {
        DefaultHighlighter {
            style: ContentStyle::default(),
        }
    }
}

impl Highlighter for DefaultHighlighter {
    fn highlight(&self, buf: &str) -> StyledBuf {
        let mut styled_buf = StyledBuf::new();

        styled_buf.push(StyledContent::new(self.style, buf.to_string()));

        styled_buf
    }
}
