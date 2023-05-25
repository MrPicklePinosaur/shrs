use std::{
    borrow::Cow,
    fmt::Display,
    io::{stdout, BufWriter, Write},
    ops::{Index, Range, RangeBounds},
};

use crossterm::{
    cursor::{self, MoveToColumn},
    style::{ContentStyle, Print, PrintStyledContent, StyledContent, Stylize},
    terminal::{self, Clear, ScrollUp},
    QueueableCommand,
};
use shrs_core::{Context, Runtime, Shell};
use unicode_width::UnicodeWidthStr;

use crate::{completion::Completion, line::LineCtx, menu::Menu, prompt::Prompt, CursorStyle};

/// Text to be renderered by painter
#[derive(Clone)]
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
    fn count_newlines(&self) -> u16 {
        let mut lines = 0;
        for span in self.spans() {
            if span.content().contains("\n") {
                lines += 1;
            }
        }
        lines
    }

    /// Length of content in characters
    ///
    /// The length returned is the 'visual' length of the character, in other words, how many
    /// terminal columns it takes up
    pub fn content_len(&self) -> usize {
        use unicode_width::UnicodeWidthStr;
        // TODO this copies the entire contents just to get the len, can probably optimize by using
        // borrowed version
        let raw = self.contents();
        UnicodeWidthStr::width(raw.as_str())
    }

    /// Return the contents of StyledBuf with just the raw characters and no formatting
    pub fn contents(&self) -> String {
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
    num_newlines: u16,
}

impl Painter {
    pub fn new() -> Self {
        Painter {
            out: BufWriter::new(stdout()),
            term_size: (0, 0),
            prompt_line: 0,
            num_newlines: 0,
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
        menu: &Box<dyn Menu<MenuItem = Completion, PreviewItem = String>>,
        styled_buf: StyledBuf,
        cursor_ind: usize,
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
        let mut total_newlines = 0;
        let prompt_left = prompt.as_ref().prompt_left(line_ctx);
        let prompt_right = prompt.as_ref().prompt_right(line_ctx);

        total_newlines += styled_buf.count_newlines();
        total_newlines += prompt_left.count_newlines();
        total_newlines += prompt_right.count_newlines();

        if self.num_newlines < total_newlines {
            self.num_newlines = total_newlines;
        }

        // clean up current line first
        self.out
            .queue(cursor::MoveTo(
                0,
                self.prompt_line.saturating_sub(self.num_newlines),
            ))?
            .queue(Clear(terminal::ClearType::FromCursorDown))?;

        // render left prompt
        let mut left_space = 0; // cursor position from left side of terminal
        left_space += prompt_left.content_len();
        for span in prompt_left.into_spans() {
            self.out.queue(PrintStyledContent(span))?;
        }

        // render line (with syntax highlight spans)
        // TODO introduce better slicing of StyledBuf
        let slice = &styled_buf.contents();
        if slice.contains("\n") {
            left_space = 0;
        }
        let chars = slice
            .as_str()
            .split("\n")
            .last()
            .unwrap()
            .chars()
            .take(cursor_ind)
            .collect::<String>();
        left_space += UnicodeWidthStr::width(chars.as_str());
        for span in styled_buf.spans() {
            let content = span.content().replace("\n", "\r\n");
            self.out
                .queue(Print(StyledContent::new(*span.style(), content)))?;
        }

        // render right prompt
        let mut right_space = self.term_size.0;
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

        // set cursor style
        let cursor_style = line_ctx.ctx.state.get_or_default::<CursorStyle>();
        self.out.queue(cursor_style.style)?;

        self.out.flush()?;

        Ok(())
    }

    pub fn newline(&mut self) -> crossterm::Result<()> {
        self.num_newlines = 0;
        self.out.queue(Print("\r\n"))?;
        self.out.flush()?;
        Ok(())
    }
}
