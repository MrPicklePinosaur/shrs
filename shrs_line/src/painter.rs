use std::io::{stdout, BufWriter, Write};

use crossterm::{
    cursor::{self, MoveUp},
    style::{Attribute, Print, SetAttribute, StyledContent, Stylize},
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ScrollUp},
    QueueableCommand,
};

use crate::{cursor::Cursor, menu::Menu, prompt::Prompt};

/// Text to be renderered by painter
pub struct TextSpan {
    spans: Vec<StyledContent<String>>,
}

impl TextSpan {
    pub fn new() -> Self {
        TextSpan { spans: vec![] }
    }

    pub fn push(&mut self, span: StyledContent<String>) {
        self.spans.push(span);
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
        buf: &str,
        cursor_ind: usize,
        cursor: &Box<dyn Cursor>,
    ) -> crossterm::Result<()> {
        self.out.queue(cursor::Hide)?;

        // scroll up if we need more lines
        if menu.is_active() {
            let required_lines = menu.items().len() as u16 + 1;
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

        // render line
        self.out
            .queue(Print(prompt.as_ref().prompt_left()))?
            .queue(Print(&buf[..cursor_ind]))?
            .queue(cursor::SavePosition)?
            .queue(Print(&buf[cursor_ind..]))?;

        // render menu
        if menu.is_active() {
            menu.unselected_style(&mut self.out)?;
            for (i, menu_item) in menu.items().iter().enumerate() {
                self.out.queue(Print("\r\n"))?;
                if menu.cursor() as usize == i {
                    menu.selected_style(&mut self.out)?;
                }

                self.out.queue(Print(menu_item))?;
                menu.unselected_style(&mut self.out)?;
            }
            self.out
                .queue(MoveUp(menu.items().len().saturating_sub(1) as u16))?;
        }

        self.out.queue(cursor::RestorePosition)?;
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
