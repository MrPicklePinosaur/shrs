use shrs::prelude::{styled_buf::StyledBuf, Highlighter};

use crate::MuxState;

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
        let mut styled_buf = StyledBuf::new(&buf);

        if let Some(mux_state) = state.ctx.state.get::<MuxState>() {
            if let Some(t) = mux_state.get_syntax_theme() {
                t.apply(&mut styled_buf);
            }
        };
        styled_buf
    }
}
