use std::io::{stdout, BufWriter, Write};

use crossterm::style::Print;
use crossterm::QueueableCommand;

pub struct OutputWriter {
    out: BufWriter<std::io::Stdout>,
}
impl Default for OutputWriter {
    fn default() -> Self {
        Self {
            out: BufWriter::new(stdout()),
        }
    }
}
impl OutputWriter {
    pub fn print(&mut self, s: String) -> anyhow::Result<()> {
        self.out.queue(Print(s))?;
        self.out.flush()?;
        Ok(())
    }
    pub fn println(&mut self, s: String) -> anyhow::Result<()> {
        self.out.queue(Print(s))?;
        self.out.queue(Print("\r\n"))?;
        self.out.flush()?;
        Ok(())
    }
}
