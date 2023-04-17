use std::io::{stdout, BufWriter, Write};

use crossterm::{
    cursor::{self},
    style::{Print, StyledContent},
    terminal::{self, Clear, ScrollUp},
    QueueableCommand,
};

use crate::{cursor::Cursor, menu::Menu, prompt::Prompt};

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

    /// Get each block of styled text
    pub fn spans(&self) -> &Vec<StyledContent<String>> {
        &self.spans
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
        prompt: impl AsRef<T>,
        menu: &Box<dyn Menu<MenuItem = String>>,
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

        // render prompt
        let mut left_space = 0; // cursor position from left side of terminal
        let prompt_left = prompt.as_ref().prompt_left();
        left_space += prompt_left.len();
        self.out.queue(Print(prompt_left))?;

        // render line (with syntax highlight spans)
        left_space += cursor_ind;
        for span in styled_buf.spans() {
            self.out.queue(Print(span))?;
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
