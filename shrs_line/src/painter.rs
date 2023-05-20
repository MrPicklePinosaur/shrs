use std::{
    borrow::Cow,
    fmt::Display,
    io::{stdout, BufWriter, Write},
    ops::{Index, Range, RangeBounds},
};

use crossterm::{
    cursor::{self, MoveToColumn},
    style::{Print, PrintStyledContent, StyledContent},
    terminal::{self, Clear, ScrollUp},
    QueueableCommand,
};
use shrs_core::{Context, Runtime, Shell};
use unicode_width::UnicodeWidthStr;

use crate::{cursor::Cursor, line::LineCtx, menu::Menu, prompt::Prompt};

/// Text to be renderered by painter
pub struct StyledBuf {
    spans: Vec<StyledContent<String>>,
}

impl StyledBuf {
    pub fn new() -> Self {
        StyledBuf { spans: vec![] }
    }

    pub fn push(&mut self, span: StyledContent<String>) {
        self.spans.push(span);
    }

    fn spans(&self) -> impl Iterator<Item = &StyledContent<String>> {
        self.spans.iter()
    }

    fn into_spans(self) -> impl IntoIterator<Item = StyledContent<String>> {
        self.spans.into_iter()
    }

    /// Length of content in characters
    ///
    /// The length returned is the 'visual' length of the character, in other words, how many
    /// terminal columns it takes up
    pub fn content_len(&self) -> usize {
        use unicode_width::UnicodeWidthStr;
        // TODO this copies the entire contents just to get the len, can probably optimize by using
        // borrowed version
        let raw = self.as_string();
        UnicodeWidthStr::width(raw.as_str())
    }

    /// Return the contents of StyledBuf with just the raw characters and no formatting
    pub fn as_string(&self) -> String {
        self.spans
            .iter()
            .map(|s| s.content().as_str())
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Display for StyledBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in self.spans() {
            write!(f, "{}", span)?;
        }
        Ok(())
    }
}

impl FromIterator<StyledContent<String>> for StyledBuf {
    fn from_iter<T: IntoIterator<Item = StyledContent<String>>>(iter: T) -> Self {
        Self {
            spans: Vec::from_iter(iter),
        }
    }
}

pub struct Painter {
    /// The output buffer
    out: BufWriter<std::io::Stdout>,
    /// Dimensions of current terminal window
    term_size: (u16, u16),
    /// Current line the prompt is on
    prompt_line: u16,
}

impl Painter {
    pub fn new() -> Self {
        Painter {
            out: BufWriter::new(stdout()),
            term_size: (0, 0),
            prompt_line: 0,
        }
    }

    /// Clear screen and move prompt to the top
    pub fn init(&mut self) -> crossterm::Result<()> {
        self.prompt_line = 0;
        self.term_size = terminal::size()?;

        // advance to next row if cursor in middle of line
        let (c, r) = cursor::position()?;
        let r = if c > 0 { r + 1 } else { r };

        self.prompt_line = r;

        Ok(())
    }

    pub fn paint<T: Prompt + ?Sized>(
        &mut self,
        line_ctx: &mut LineCtx,
        prompt: impl AsRef<T>,
        menu: &Box<dyn Menu<MenuItem = String, PreviewItem = String>>,
        styled_buf: StyledBuf,
        cursor_ind: usize,
        cursor: &Box<dyn Cursor>,
    ) -> anyhow::Result<()> {
        self.out.queue(cursor::Hide)?;

        // scroll up if we need more lines
        if menu.is_active() {
            let required_lines = menu.required_lines() as u16;
            let remaining_lines = self.term_size.1.saturating_sub(self.prompt_line);
            if required_lines > remaining_lines {
                let extra_lines = required_lines.saturating_sub(remaining_lines);
                self.out.queue(ScrollUp(extra_lines.try_into().unwrap()))?;
                self.prompt_line = self.prompt_line.saturating_sub(extra_lines);
            }
        }

        // clean up current line first
        self.out
            .queue(cursor::MoveTo(0, self.prompt_line))?
            .queue(Clear(terminal::ClearType::FromCursorDown))?;

        // render left prompt
        let mut left_space = 0; // cursor position from left side of terminal
        let prompt_left = prompt.as_ref().prompt_left(line_ctx);
        left_space += prompt_left.content_len();
        for span in prompt_left.into_spans() {
            self.out.queue(PrintStyledContent(span))?;
        }

        // render line (with syntax highlight spans)
        // TODO introduce better slicing of StyledBuf
        let slice = &styled_buf.as_string();
        let chars = slice.as_str().chars().take(cursor_ind).collect::<String>();
        left_space += UnicodeWidthStr::width(chars.as_str());
        for span in styled_buf.spans() {
            self.out.queue(Print(span))?;
        }

        // render right prompt
        let mut right_space = self.term_size.0;
        let prompt_right = prompt.as_ref().prompt_right(line_ctx);
        right_space -= prompt_right.content_len() as u16;
        self.out.queue(MoveToColumn(right_space))?;
        for span in prompt_right.into_spans() {
            self.out.queue(PrintStyledContent(span))?;
        }

        // render menu
        if menu.is_active() {
            menu.render(&mut self.out)?;
        }

        // self.out.queue(cursor::RestorePosition)?;
        self.out.queue(cursor::MoveToColumn(left_space as u16))?;
        self.out.queue(cursor::Show)?;
        self.out.queue(cursor.get_cursor())?;
        self.out.flush()?;

        Ok(())
    }

    pub fn newline(&mut self) -> crossterm::Result<()> {
        self.out.queue(Print("\r\n"))?;
        self.out.flush()?;
        Ok(())
    }
}
