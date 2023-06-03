//! Internal renderer

use std::{
    collections::HashMap,
    fmt::Display,
    io::{stdout, BufWriter, Stdout, Write},
};

use crossterm::{
    cursor::{self, MoveToColumn},
    style::{ContentStyle, Print, PrintStyledContent, StyledContent},
    terminal::{self, Clear, ScrollUp},
    QueueableCommand,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    completion::Completion, cursor::CursorStyle, line::LineCtx, menu::Menu, prompt::Prompt,
};

/// Text to be renderered by painter
#[derive(Clone)]
pub struct StyledBuf {
    content: String,
    styles: Vec<ContentStyle>,
}

impl StyledBuf {
    pub fn empty() -> Self {
        Self {
            content: "".to_string(),
            styles: vec![],
        }
    }
    pub fn new(content: &str, style: ContentStyle) -> Self {
        Self {
            content: content.to_string(),
            styles: vec![style; content.chars().count()],
        }
    }

    pub fn push(&mut self, content: &str, style: ContentStyle) {
        self.content += content;

        for _ in content.chars() {
            self.styles.push(style);
        }
    }

    fn spans(&self) -> Vec<StyledContent<String>> {
        let mut x: Vec<StyledContent<String>> = vec![];
        for (i, c) in self.content.chars().enumerate() {
            x.push(StyledContent::new(self.styles[i], c.to_string()));
        }
        x
    }

    fn count_newlines(&self) -> u16 {
        self.content
            .chars()
            .into_iter()
            .filter(|c| *c == '\n')
            .count()
            .try_into()
            .unwrap()
    }

    /// Length of content in characters
    ///
    /// The length returned is the 'visual' length of the character, in other words, how many
    /// terminal columns it takes up
    pub fn content_len(&self) -> usize {
        use unicode_width::UnicodeWidthStr;
        UnicodeWidthStr::width(self.content.as_str())
    }

    pub fn change_style(&mut self, c_style: HashMap<usize, ContentStyle>, offset: usize) {
        for (u, s) in c_style.into_iter() {
            if offset <= u {
                self.styles[u - offset] = s;
            }
        }
    }
}

impl Display for StyledBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StyledBuf {}", self.content);
        Ok(())
    }
}

impl FromIterator<StyledContent<String>> for StyledBuf {
    fn from_iter<T: IntoIterator<Item = StyledContent<String>>>(iter: T) -> Self {
        let mut buf = Self::empty();
        for i in iter {
            buf.push(i.content(), i.style().to_owned());
        }
        buf
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
        let mut prompt_right_rendered = false;
        let render_prompt_right = |out: &mut BufWriter<Stdout>| -> anyhow::Result<()> {
            let mut right_space = self.term_size.0;
            right_space -= prompt_right.content_len() as u16;
            out.queue(MoveToColumn(right_space))?;
            for span in prompt_right.spans() {
                out.queue(PrintStyledContent(span))?;
            }
            Ok(())
        };

        total_newlines += styled_buf.count_newlines();
        total_newlines += prompt_left.count_newlines();

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

        for span in prompt_left.spans() {
            self.out.queue(PrintStyledContent(span))?;
        }
        //prompt space doesnt matter if there is going to be a carriage return
        if total_newlines == 0 {
            left_space += prompt_left.content_len();
        }

        //take last line of buf and get length for cursor
        let chars = styled_buf
            .content
            .as_str()
            .split('\n')
            .last()
            .unwrap()
            .chars()
            .take(cursor_ind)
            .collect::<String>();
        left_space += UnicodeWidthStr::width(chars.as_str());
        for span in styled_buf.spans() {
            let content = span.content();
            if span.content() == "\n" {
                if !prompt_right_rendered {
                    render_prompt_right(&mut self.out)?;
                }
                prompt_right_rendered = true;
                self.out.queue(Print("\r"))?;
            }
            self.out
                .queue(Print(StyledContent::new(*span.style(), content)))?;
        }
        if !prompt_right_rendered {
            render_prompt_right(&mut self.out)?;
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
