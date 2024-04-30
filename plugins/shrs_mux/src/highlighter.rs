use shrs::prelude::{styled_buf::StyledBuf, *};

use crate::MuxState;

pub struct MuxHighlighter {}
impl MuxHighlighter {
    pub fn new() -> Self {
        Self {}
    }
}
impl Highlighter for MuxHighlighter {
    fn highlight(&self, sh: &Shell, states: &States, buf: &String) -> anyhow::Result<StyledBuf> {
        let mut styled_buf = StyledBuf::new(&buf);

        if let Ok(mux_state) = states.try_get::<MuxState>() {
            if let Some(t) = mux_state.get_syntax_theme() {
                t.apply(&mut styled_buf);
            }
        };
        Ok(styled_buf)
    }
}
