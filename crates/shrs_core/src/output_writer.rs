//! Writer for printing to stdout and stderr

use std::{
    fmt::Display,
    io::{stderr, stdout, BufWriter, Write},
};

use crossterm::{
    style::{ContentStyle, PrintStyledContent, Stylize},
    QueueableCommand,
};
use shrs_utils::StyledBuf;
/// Writer for printing to stdout and stderr
///
/// Printing in handlers should be done through `OutputWriter`,
/// which automatically uses the configured out and err colors.
/// It also records output in commands so that it can be collected into `CmdOutput`
/// ```
/// # use shrs_core::prelude::*;
/// fn hello(mut out: StateMut<OutputWriter>) -> anyhow::Result<()> {
///     out.println("Hello")?;
///     Ok(())
/// }
/// ```

pub struct OutputWriter {
    stdout: BufWriter<std::io::Stdout>,
    stderr: BufWriter<std::io::Stderr>,
    collecting: bool,
    out: String,
    err: String,
    out_style: ContentStyle,
    err_style: ContentStyle,
}
impl OutputWriter {
    pub fn new(out_style: ContentStyle, err_style: ContentStyle) -> Self {
        Self {
            out_style,
            err_style,
            stdout: BufWriter::new(stdout()),
            stderr: BufWriter::new(stderr()),
            collecting: false,
            out: String::new(),
            err: String::new(),
        }
    }

    pub(crate) fn begin_collecting(&mut self) {
        self.collecting = true;
    }

    /// Prints to stderr and appends a newline character
    pub fn eprint<T: Display>(&mut self, s: T) -> anyhow::Result<()> {
        if self.collecting {
            self.err.push_str(s.to_string().as_str());
        }

        self.stderr
            .queue(PrintStyledContent(self.err_style.apply(s.to_string())))?;
        self.stderr.flush()?;
        Ok(())
    }

    /// Calls eprint, then prints a newline
    pub fn eprintln<T: Display>(&mut self, s: T) -> anyhow::Result<()> {
        self.eprint(s)?;
        self.eprint("\r\n")?;
        Ok(())
    }

    /// Prints to stdout using out_style for styling.
    pub fn print<T: Display>(&mut self, s: T) -> anyhow::Result<()> {
        if self.collecting {
            self.out.push_str(s.to_string().as_str());
        }
        self.stdout
            .queue(PrintStyledContent(self.out_style.apply(s.to_string())))?;
        self.stdout.flush()?;
        Ok(())
    }
    ///Calls print, then prints a newline.
    pub fn println<T: Display>(&mut self, s: T) -> anyhow::Result<()> {
        self.print(s)?;
        self.print("\r\n")?;
        Ok(())
    }

    pub(crate) fn end_collecting(&mut self) -> (String, String) {
        self.collecting = false;
        (self.out.drain(..).collect(), self.err.drain(..).collect())
    }
    /// Prints a `StyledBuf` to stdout.
    /// If there are multiple lines, if will print \r\n between them.
    pub fn print_buf(&mut self, buf: StyledBuf) -> anyhow::Result<()> {
        let lines = buf.lines();

        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                self.print("\r\n")?;
            }
            for span in line {
                self.stdout.queue(PrintStyledContent(span.clone()))?;
            }
        }
        self.stdout.flush()?;

        Ok(())
    }
}
impl Default for OutputWriter {
    fn default() -> Self {
        Self::new(ContentStyle::new().white(), ContentStyle::new().red())
    }
}
