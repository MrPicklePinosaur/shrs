//! General purpose selection menu for shell

use std::{cmp::Ordering, fmt::Display};

use crossterm::{
    cursor::{MoveDown, MoveToColumn, MoveUp},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    QueueableCommand,
};

use crate::{completion::Completion, painter::Painter};

pub type Out = std::io::BufWriter<std::io::Stdout>;

/// Implement this trait to define your own menu
pub trait Menu {
    type MenuItem;
    type PreviewItem: Display;

    /// Go to the next selection
    fn next(&mut self);
    /// Go to the previous selection
    fn previous(&mut self);
    /// Accept the current selection
    fn accept(&mut self) -> Option<&Self::MenuItem>;
    /// Get the current selection
    fn current_selection(&self) -> Option<&Self::MenuItem>;
    /// Get the position of the cursor
    fn cursor(&self) -> u32;
    /// Check if menu is currently active
    fn is_active(&self) -> bool;
    /// Activate the menu
    fn activate(&mut self);
    /// Disactivate the menu
    fn disactivate(&mut self);
    /// Get the current items in the menu
    fn items(&self) -> Vec<&(Self::PreviewItem, Self::MenuItem)>;
    fn set_items(&mut self, items: Vec<(Self::PreviewItem, Self::MenuItem)>);

    fn render(&self, out: &mut Out, painter: &Painter) -> anyhow::Result<()>;
    fn required_lines(&self, painter: &Painter) -> usize;
}

pub type SortFn = fn(&(String, Completion), &(String, Completion)) -> Ordering;

/// Simple menu that prompts user for a selection
pub struct DefaultMenu {
    selections: Vec<(String, Completion)>,
    /// Currently selected item
    cursor: u32,
    active: bool,
    column_padding: usize,
    /// Max length in characters that the comment message is allowed to take up
    comment_max_length: usize,
    /// Max number of entries to show when rendering the menu
    _limit: usize,
    /// Function to use to sort the entries
    // TODO can we make this vary depending on which completions are used? does sorting belong more
    // to completion?
    sort: SortFn,
}

impl Default for DefaultMenu {
    fn default() -> Self {
        DefaultMenu {
            selections: vec![],
            cursor: 0,
            active: false,
            comment_max_length: 30,
            column_padding: 2,
            _limit: 20,
            // by default sort alphabetical by display name
            sort: |a, b| -> Ordering { a.0.to_lowercase().cmp(&b.0.to_lowercase()) },
        }
    }
}

impl DefaultMenu {
    pub fn new_with_limit(_limit: usize) -> Self {
        Self {
            _limit,
            ..Default::default()
        }
    }

    // TODO make these configurable?
    fn selected_style(&self, out: &mut Out) -> crossterm::Result<()> {
        execute!(
            out,
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black),
        )?;
        Ok(())
    }

    fn unselected_style(&self, out: &mut Out) -> crossterm::Result<()> {
        execute!(out, ResetColor)?;
        Ok(())
    }

    fn comment_style(&self, out: &mut Out) -> crossterm::Result<()> {
        execute!(out, SetForegroundColor(Color::Yellow),)?;
        Ok(())
    }

    fn max_width(&self) -> usize {
        // first determine how many columns are needed to list all completions
        let mut max_width = 0;
        for menu_item in self.items() {
            // extra +4 is for formatting characters around the comment
            let comment_len = menu_item
                .1
                .comment
                .as_ref()
                .map(|comment| comment.len().min(self.comment_max_length) + 4)
                .unwrap_or(0);
            let menu_item_len = menu_item.0.len() + comment_len;

            max_width = max_width.max(menu_item_len);
        }

        max_width
    }
}

impl Menu for DefaultMenu {
    type MenuItem = Completion;
    type PreviewItem = String;

    fn next(&mut self) {
        if self.cursor as usize == self.selections.len().saturating_sub(1) {
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }
    }
    fn previous(&mut self) {
        if self.cursor == 0 {
            self.cursor = self.selections.len().saturating_sub(1) as u32;
        } else {
            self.cursor = self.cursor.saturating_sub(1);
        }
    }
    fn accept(&mut self) -> Option<&Self::MenuItem> {
        self.disactivate();
        self.current_selection()
    }
    fn current_selection(&self) -> Option<&Self::MenuItem> {
        self.selections.get(self.cursor as usize).map(|x| &x.1)
    }
    fn cursor(&self) -> u32 {
        self.cursor
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn activate(&mut self) {
        // dont activate if menu is empty
        self.active = !self.selections.is_empty();
    }
    fn disactivate(&mut self) {
        self.active = false;
    }
    fn items(&self) -> Vec<&(Self::PreviewItem, Self::MenuItem)> {
        // TODO is this the right way to case Vec<String> to Vec<&String> ??
        self.selections.iter().collect()
    }
    fn set_items(&mut self, mut items: Vec<(Self::PreviewItem, Self::MenuItem)>) {
        self.selections.clear();
        items.sort_by(self.sort);
        self.selections.append(&mut items);
        self.cursor = 0;
    }

    fn render(&self, out: &mut Out, painter: &Painter) -> anyhow::Result<()> {
        let mut i = 0;
        let mut column_start: usize = 0;

        let max_width = self.max_width();
        let mut columns_needed = painter.get_term_size().0 as usize / max_width;

        // terminal is not wide enough to render even one line
        if columns_needed == 0 {
            columns_needed = 1;
        }

        // ceil division
        let rows_needed = (self.items().len() + columns_needed - 1) / columns_needed;

        self.unselected_style(out)?;
        for column in self.items().chunks(rows_needed) {
            // length of the longest word in column
            let mut longest_word = 0;

            for menu_item in column.iter() {
                longest_word = longest_word.max(menu_item.0.len());
                out.queue(MoveDown(1))?;
                out.queue(MoveToColumn(column_start as u16))?;
                if self.cursor() as usize == i {
                    self.selected_style(out)?;
                }
                out.queue(Print(&menu_item.0))?;
                self.unselected_style(out)?;

                if let Some(comment) = &menu_item.1.comment {
                    let comment_len = comment.len().min(self.comment_max_length);
                    out.queue(MoveToColumn(
                        (column_start + max_width - comment_len - 2) as u16, // -2 for parentheses
                    ))?;
                    out.queue(Print("("))?;
                    self.comment_style(out)?;
                    out.queue(Print(truncate(comment, comment_len)))?;
                    self.unselected_style(out)?;
                    out.queue(Print(")"))?;
                }

                i += 1;
            }
            column_start += max_width + self.column_padding;

            // move back up
            out.queue(MoveUp(column.len() as u16))?;
        }

        Ok(())
    }

    fn required_lines(&self, painter: &Painter) -> usize {
        // TODO a bit of duplicated code

        let max_width = self.max_width();

        let mut columns_needed = painter.get_term_size().0 as usize / max_width;

        // terminal is not wide enough to render even one line
        if columns_needed == 0 {
            columns_needed = 1;
        }

        // ceil division
        let rows_needed = (self.items().len() + columns_needed - 1) / columns_needed;

        rows_needed + 1
    }
}

/// Utility to truncate string and insert ellipses at end
fn truncate(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => s.to_string(),
        Some((idx, _)) => {
            let mut truncated = s[..idx.saturating_sub(3)].to_string();
            truncated.push_str("...");
            truncated
        },
    }
}
