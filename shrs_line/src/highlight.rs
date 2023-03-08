pub trait Highlight {}

pub struct DefaultHighlight {}

impl DefaultHighlight {
    pub fn new() -> Self {
        DefaultHighlight {}
    }
}

impl Highlight for DefaultHighlight {}
