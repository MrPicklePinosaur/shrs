use std::io::{stdout, BufWriter, Write};

use crossterm::{
    cursor,
    style::{Attribute, Print, SetAttribute},
    terminal::{self, Clear},
    QueueableCommand,
};

use crate::{cursor::Cursor, menu::Menu, prompt::Prompt};

pub struct Painter {
    /// The output buffer
    out: BufWriter<std::io::Stdout>,
    /// Dimensions of current terminal window
    term_size: (u16, u16),
    /// Current line the prompt is on
    prompt_line: u16,
}

impl Painter {
    pub fn new() -> crossterm::Result<Self> {
        let term_size = terminal::size()?;
        Ok(Painter {
            out: BufWriter::new(stdout()),
            term_size,
            prompt_line: 0,
        })
    }

    /// Clear screen and move prompt to the top
    pub fn init<T: Prompt + ?Sized>(
        &mut self,
        _prompt: impl AsRef<T>,
        _menu: &Box<dyn Menu<MenuItem = String>>,
    ) -> crossterm::Result<()> {
        self.out.queue(Clear(terminal::ClearType::All))?;
        self.prompt_line = 0;

        self.out.flush()?;

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

        // clean up current line first
        let cursor_pos = cursor::position()?;
        self.out
            .queue(cursor::MoveTo(0, cursor_pos.1))?
            .queue(Clear(terminal::ClearType::FromCursorDown))?;

        // render line
        self.out
            .queue(Print(prompt.as_ref().prompt_left()))?
            .queue(Print(&buf[..cursor_ind]))?
            .queue(cursor::SavePosition)?
            .queue(Print(&buf[cursor_ind..]))?;

        // render menu
        if menu.is_active() {
            self.out.queue(Print("\r\n"))?;
            for (i, menu_item) in menu.items().iter().enumerate() {
                if menu.cursor() == i as i32 {
                    self.out.queue(SetAttribute(Attribute::Bold))?;
                }

                self.out.queue(Print(menu_item))?.queue(Print("\r\n"))?;

                self.out.queue(SetAttribute(Attribute::NoBold))?;
            }
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
