use std::collections::HashMap;

use crossterm::style::ContentStyle;
use rustpython_parser::{lexer::lex, Mode, StringKind, Tok};
use shrs::{
    anyhow,
    crossterm::Stylize,
    prelude::{styled_buf::StyledBuf, Context, Highlighter, Runtime, Shell},
    readline::SyntaxTheme,
};

use crate::{ChangeLangCtx, MuxState};

pub struct MuxHighlighter {}
impl MuxHighlighter {
    pub fn new() -> Self {
        Self {}
    }
}
impl Highlighter for MuxHighlighter {
    fn highlight(
        &self,
        state: &shrs::prelude::LineStateBundle,
        buf: &str,
    ) -> shrs::prelude::styled_buf::StyledBuf {
        let mut styled_buf = StyledBuf::new(&buf, ContentStyle::new());

        if let Some(mux_state) = state.ctx.state.get::<MuxState>() {
            if let Some(t) = mux_state.get_syntax_theme() {
                t.apply(&mut styled_buf);
            }
        };
        styled_buf
    }
}
