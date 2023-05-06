//! Builder pattern for prompt
//!
//!

#[macro_use]
extern crate derive_builder;

pub trait Section {
    fn render(&self);
}

pub struct DerivePromptBuilder {
    sections: Vec<Box<dyn Section>>,
}

impl DerivePromptBuilder {
    pub fn new() -> Self {
        DerivePromptBuilder {
            sections: Vec::new(),
        }
    }

    /// Display section conditionally
    pub fn cond(&mut self, section: impl Section, pred: fn() -> bool) {}

    /*
    /// Insert a text section
    pub fn text(&mut self, section: Section) {

    }

    /// Insert newline
    pub fn newline(&mut self) {

    }

    /// Insert some spacing
    pub fn spacer(&mut self, width: usize) {

    }
    */
}
