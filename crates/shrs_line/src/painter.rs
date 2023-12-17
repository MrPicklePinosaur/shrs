//! Internal renderer

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{stdout, BufRead, BufWriter, Stdout, Write},
    iter,
};

use crossterm::{
    cursor::{self, MoveToColumn, MoveToNextLine, MoveToPreviousLine},
    style::{ContentStyle, Print, PrintStyledContent, StyledContent},
    terminal::{self, Clear, ScrollUp},
    QueueableCommand,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    completion::Completion, cursor::CursorStyle, line::LineCtx, menu::Menu, prompt::Prompt,
};
/// Text to be rendered by painter
#[derive(Clone)]
pub struct StyledBuf {
    content: String,
    styles: Vec<ContentStyle>,
}

impl StyledBuf {
    pub fn empty() -> Self {
        Self {
            content: String::new(),
            styles: vec![],
        }
    }
    pub fn new(content: &str, style: ContentStyle) -> Self {
        let mut s = Self::empty();
        s.push(content, style);
        s
    }

    pub fn push(&mut self, content: &str, style: ContentStyle) {
        self.content += content;

        for _ in content.chars() {
            self.styles.push(style);
        }
    }

    pub fn lines(&self) -> Vec<Vec<StyledContent<String>>> {
        let mut lines: Vec<Vec<StyledContent<String>>> = vec![];
        let mut i = 0;
        for line in self.content.split("\n") {
            let mut x: Vec<StyledContent<String>> = vec![];

            for c in line.chars() {
                x.push(StyledContent::new(self.styles[i], c.to_string()));
                i += 1;
            }
            i += 1;
            lines.push(x);
        }
        lines
    }
    //can be simply changed to just the len(lines())-1
    //kept for now
    pub fn count_newlines(&self) -> u16 {
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
    pub fn content_len(&self) -> u16 {
        use unicode_width::UnicodeWidthStr;
        UnicodeWidthStr::width(self.content.as_str()) as u16
    }

    pub fn change_style(&mut self, c_style: HashMap<usize, ContentStyle>, offset: usize) {
        for (u, s) in c_style.into_iter() {
            if offset <= u {
                self.styles[u - offset] = s;
            }
        }
    }
}
pub fn line_content_len(line: Vec<StyledContent<String>>) -> u16 {
    let c = line
        .iter()
        .map(|x| x.content().as_str())
        .collect::<String>();
    UnicodeWidthStr::width(c.as_str()) as u16
}

impl Display for StyledBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content);
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
    out: RefCell<BufWriter<std::io::Stdout>>,
    /// Dimensions of current terminal window
    term_size: (u16, u16),
    /// Current line the prompt is on
    prompt_line: u16,
    num_newlines: u16,
}

impl Painter {
    pub fn new() -> Self {
        Painter {
            out: RefCell::new(BufWriter::new(stdout())),
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

    pub fn set_term_size(&mut self, w: u16, h: u16) {
        self.term_size = (w, h);
    }

    pub fn get_term_size(&self) -> (u16, u16) {
        self.term_size
    }

    pub fn paint<T: Prompt + ?Sized>(
        &mut self,
        line_ctx: &mut LineCtx,
        prompt: impl AsRef<T>,
        menu: &Box<dyn Menu<MenuItem = Completion, PreviewItem = String>>,
        styled_buf: &StyledBuf,
        cursor_ind: usize,
    ) -> anyhow::Result<()> {
        self.out.borrow_mut().queue(cursor::Hide)?;

        // scroll up if we need more lines
        if menu.is_active() {
            let required_lines = menu.required_lines(&self) as u16;
            let remaining_lines = self.term_size.1.saturating_sub(self.prompt_line);
            if required_lines > remaining_lines {
                let extra_lines = required_lines.saturating_sub(remaining_lines);
                self.out
                    .borrow_mut()
                    .queue(ScrollUp(extra_lines.try_into().unwrap()))?;
                self.prompt_line = self.prompt_line.saturating_sub(extra_lines);
            }
        }
        //newlines to account for when clearing and printing prompt
        let mut total_newlines = 0;
        let prompt_left = prompt.as_ref().prompt_left(line_ctx);
        let prompt_right = prompt.as_ref().prompt_right(line_ctx);
        let prompt_left_lines = prompt_left.lines();
        let prompt_right_lines = prompt_right.lines();
        let styled_buf_lines = styled_buf.lines();

        //need to also take into account extra lines needed for prompt_right
        total_newlines += styled_buf_lines.len() - 1;
        total_newlines += prompt_left_lines.len() - 1;

        //make sure num_newlines never gets smaller, and prompt never moves down
        if self.num_newlines < total_newlines as u16 {
            self.num_newlines = total_newlines as u16;
        }

        // clean up current line first
        self.out
            .borrow_mut()
            .queue(cursor::MoveTo(
                0,
                self.prompt_line.saturating_sub(self.num_newlines),
            ))?
            .queue(Clear(terminal::ClearType::FromCursorDown))?;

        // cursor position from left side of terminal
        let mut left_space = 0;

        let mut ri = 0;
        let mut li = 0;
        let mut bi = 0;
        //RENDER PROMPT
        //loop through lines rendering prompt left and prompt right and only start rendering of
        //buffer when prompt_left is out of lines

        loop {
            if li < prompt_left_lines.len() {
                for span in prompt_left_lines[li].iter() {
                    self.out
                        .borrow_mut()
                        .queue(PrintStyledContent(span.clone()))?;
                }

                li += 1;
            }
            if bi < styled_buf_lines.len() && li >= prompt_left_lines.len() {
                for span in styled_buf_lines[bi].iter() {
                    self.out
                        .borrow_mut()
                        .queue(PrintStyledContent(span.clone()))?;
                }
                bi += 1;
            }

            if ri < prompt_right_lines.len() {
                let mut right_space = self.term_size.0;
                right_space -= line_content_len(prompt_right_lines[ri].clone()) as u16;
                self.out.borrow_mut().queue(MoveToColumn(right_space))?;
                for span in prompt_right_lines[ri].iter() {
                    self.out
                        .borrow_mut()
                        .queue(PrintStyledContent(span.clone()))?;
                }

                ri += 1;
            }
            //no lines left in any of the styledbufs
            if li >= prompt_left_lines.len()
                && bi >= styled_buf_lines.len()
                && ri >= prompt_right_lines.len()
            {
                break;
            }
            self.out.borrow_mut().queue(MoveToNextLine(1))?;
        }

        //calculate left space
        //prompt space is 0 if there is going to be a newline in the styled_buf
        if styled_buf_lines.len().saturating_sub(1) == 0 {
            //space is width of last line of prompt_left
            left_space += UnicodeWidthStr::width(
                prompt_left
                    .content
                    .split('\n')
                    .last()
                    .unwrap()
                    .chars()
                    .collect::<String>()
                    .as_str(),
            );
        }

        //take last line of buf to the cursor index and get length for cursor
        let chars = styled_buf
            .content
            .split('\n')
            .last()
            .unwrap()
            .chars()
            .take(cursor_ind)
            .collect::<String>();
        left_space += UnicodeWidthStr::width(chars.as_str());

        // render menu
        if menu.is_active() {
            menu.render(&mut self.out.borrow_mut(), &self)?;
        }

        //move cursor to correct position
        self.out
            .borrow_mut()
            .queue(cursor::MoveToColumn(left_space as u16))?;
        self.out.borrow_mut().queue(cursor::Show)?;

        // set cursor style
        let cursor_style = line_ctx.ctx.state.get_or_default::<CursorStyle>();
        self.out.borrow_mut().queue(cursor_style.style)?;

        self.out.borrow_mut().flush()?;

        Ok(())
    }

    pub fn newline(&mut self) -> crossterm::Result<()> {
        self.out.borrow_mut().queue(Print("\r\n"))?;
        self.out.borrow_mut().flush()?;
        Ok(())
    }
}
