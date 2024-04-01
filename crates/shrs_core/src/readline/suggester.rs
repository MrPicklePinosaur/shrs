use super::line::LineStateBundle;

pub trait Suggester {
    fn suggest(&self, line_ctx: &LineStateBundle) -> Option<String>;
}
pub struct DefaultSuggester;
impl Suggester for DefaultSuggester {
    fn suggest(&self, line_ctx: &LineStateBundle) -> Option<String> {
        let h = &line_ctx.ctx.history;
        let res = line_ctx.line.get_full_command();
        if res.is_empty() {
            return None;
        }

        for i in 0..h.len() {
            if let Some(s) = h.get(i) {
                if s.starts_with(&res) {
                    return Some(s.to_owned());
                }
            }
        }
        None
    }
}
