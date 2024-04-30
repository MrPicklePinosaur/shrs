//! Core readline configuration

use std::io::{Read, Seek, Write};

use ::crossterm::{
    event::{
        read, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    style::{Color, ContentStyle},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use line::menu::DefaultMenuState;
use pino_deref::{Deref, DerefMut};
use shrs_utils::cursor_buffer::{CursorBuffer, Location};
use shrs_vi::{Action, Command, Motion, Parser};

use super::{painter::Painter, *};
use crate::{
    prelude::{Completer, Completion, CompletionCtx, History, ReplaceMethod, Shell, Theme},
    prompt_content_queue::PromptContentQueue,
    state::States,
};

pub trait Readline {
    fn read_line(&mut self, sh: &mut Shell, states: &mut States) -> String;
}

/// Operating mode of readline
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LineMode {
    /// Vi insert mode
    Insert,
    /// Vi normal mode
    Normal,
}

/// State or where the prompt is in history browse mode
#[derive(Debug, PartialEq, Eq)]
pub enum HistoryInd {
    /// Brand new prompt
    Prompt,
    /// In history line
    Line(usize),
}

impl HistoryInd {
    /// Go up (less recent) in history, if in prompt mode, then enter history
    pub fn up(&self, limit: usize) -> HistoryInd {
        match self {
            HistoryInd::Prompt => {
                if limit == 0 {
                    HistoryInd::Prompt
                } else {
                    HistoryInd::Line(0)
                }
            },
            HistoryInd::Line(i) => HistoryInd::Line((i + 1).min(limit - 1)),
        }
    }

    /// Go down (more recent) in history, if in most recent history line, enter prompt mode
    pub fn down(&self) -> HistoryInd {
        match self {
            HistoryInd::Prompt => HistoryInd::Prompt,
            HistoryInd::Line(i) => {
                if *i == 0 {
                    HistoryInd::Prompt
                } else {
                    HistoryInd::Line(i.saturating_sub(1))
                }
            },
        }
    }
}

///Current word cursor is on
#[derive(Deref, DerefMut)]
pub struct CurrentWord(String);
/// Line contents that were present before entering history mode
#[derive(Deref, DerefMut)]
pub struct SavedLine(String);

/// Line contents of the current prompt
pub struct LineContents {
    /// Cursor buffer structure for interactive editing
    pub cb: CursorBuffer,
    /// stored lines in a multiprompt command
    pub lines: String,
}

impl LineContents {
    pub fn new() -> Self {
        LineContents {
            cb: CursorBuffer::default(),
            lines: String::new(),
        }
    }

    /// Get the contents of the prompt
    pub fn get_full_command(&self) -> String {
        let mut res: String = self.lines.clone();
        let cur_line: String = self.cb.as_str().into();
        res += cur_line.as_str();

        res
    }
}

/// Configuration for readline
pub struct Line {
    painter: Painter,

    /// Currently pressed keys in normal mode
    normal_keys: String,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            painter: Painter::default(),
            normal_keys: String::new(),
        }
    }
}

impl Readline for Line {
    /// Start readline and read one line of user input
    fn read_line(&mut self, sh: &mut Shell, states: &mut States) -> String {
        states.insert(CurrentWord(String::new()));
        states.insert(HistoryInd::Prompt);
        states.insert(SavedLine(String::new()));
        states.insert(LineMode::Insert);
        states.insert(LineContents::new());

        self.read_events(sh, states).unwrap()
    }
}

impl Line {
    fn read_events(&mut self, sh: &mut Shell, states: &mut States) -> anyhow::Result<String> {
        // ensure we are always cleaning up whenever we leave this scope
        struct CleanUp;
        impl Drop for CleanUp {
            fn drop(&mut self) {
                let _ = disable_raw_mode();
                let _ = execute!(std::io::stdout(), DisableBracketedPaste);
            }
        }
        let _cleanup = CleanUp;

        enable_raw_mode()?;
        execute!(std::io::stdout(), EnableBracketedPaste)?;

        let mut auto_run = false;
        self.painter.init().unwrap();
        if let Some(c) = states.get_mut::<PromptContentQueue>().pop() {
            auto_run = c.auto_run;
            states
                .get_mut::<LineContents>()
                .cb
                .insert(Location::Cursor(), c.content.as_str())?;
        }

        loop {
            let res = states.get::<LineContents>().get_full_command();

            // syntax highlight
            let mut styled_buf = sh
                .highlighter
                .highlight(sh, states, &res)?
                .slice_from(states.get::<LineContents>().lines.len());

            // add currently selected completion to buf
            if states.get::<DefaultMenuState>().is_active() {
                if let Some(selection) = states.get::<DefaultMenuState>().current_selection() {
                    let trimmed_selection =
                        &selection.accept()[states.get::<CurrentWord>().len()..];
                    styled_buf.push(
                        trimmed_selection,
                        ContentStyle {
                            foreground_color: Some(Color::Red),
                            ..Default::default()
                        },
                    );
                }
            } else {
                if let Some(suggestion) = sh.suggester.suggest(sh, states) {
                    let trimmed_selection = suggestion[res.len()..].to_string();
                    styled_buf.push(
                        trimmed_selection.as_str(),
                        states.get::<Theme>().suggestion_style,
                    );
                }
            }

            self.painter
                .paint(states, sh, &states.get::<DefaultMenuState>(), &styled_buf)?;
            if auto_run {
                states.get_mut::<Box<dyn BufferHistory>>().clear();
                self.painter.newline()?;
                break;
            }

            let event = read()?;

            if let Event::Key(key_event) = event {
                if sh.keybindings.handle_key_event(sh, states, key_event) {
                    break;
                }
            }

            let should_break = self.handle_standard_keys(sh, states, event.clone())?;
            if should_break {
                break;
            }

            // handle menu events
            if states.get::<DefaultMenuState>().is_active() {
                self.handle_menu_keys(sh, states, event.clone())?;
            } else {
                let mode = *states.get::<LineMode>();
                match mode {
                    LineMode::Insert => self.handle_insert_keys(sh, states, event)?,

                    LineMode::Normal => self.handle_normal_keys(sh, states, event)?,
                }
            }
        }

        let res = states.get::<LineContents>().get_full_command();
        if !res.is_empty() {
            sh.history.add(sh, states, res.clone());
        }
        Ok(res)
    }

    fn handle_menu_keys(
        &mut self,
        sh: &mut Shell,
        states: &mut States,
        event: Event,
    ) -> anyhow::Result<()> {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if let Some(accepted) = states.get_mut::<DefaultMenuState>().accept().cloned() {
                    self.accept_completion(states, accepted)?;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                states.get_mut::<DefaultMenuState>().disactivate();
            },
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::SHIFT,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                states.get_mut::<DefaultMenuState>().previous();
            },
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                states.get_mut::<DefaultMenuState>().next();
            },
            _ => {
                states.get_mut::<DefaultMenuState>().disactivate();
                let mode = *states.get::<LineMode>();
                match mode {
                    LineMode::Insert => {
                        self.handle_insert_keys(sh, states, event)?;
                    },
                    LineMode::Normal => {
                        self.handle_normal_keys(sh, states, event)?;
                    },
                };
            },
        };
        Ok(())
    }

    //Keys that are universal regardless of mode, ex. Enter, Ctrl-c
    fn handle_standard_keys(
        &mut self,
        sh: &Shell,
        states: &mut States,
        event: Event,
    ) -> anyhow::Result<bool> {
        match event {
            Event::Resize(a, b) => {
                self.painter.set_term_size(a, b);
            },
            Event::Paste(p) => {
                states
                    .get_mut::<LineContents>()
                    .cb
                    .insert(Location::Cursor(), p.as_str())?;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                states.get_mut::<LineContents>().cb.clear();
                states.get_mut::<Box<dyn BufferHistory>>().clear();
                states.get_mut::<LineContents>().lines = String::new();
                self.painter.newline()?;

                return Ok(true);
            },
            // Insert suggestion when right arrow
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if let Some(suggestion) = sh.suggester.suggest(sh, states) {
                    states.get_mut::<LineContents>().cb.clear();
                    states
                        .get_mut::<LineContents>()
                        .cb
                        .insert(Location::Cursor(), suggestion.as_str())?;
                }
            },

            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                if states.get::<DefaultMenuState>().is_active() {
                    return Ok(false);
                }
                states.get_mut::<Box<dyn BufferHistory>>().clear();
                self.painter.newline()?;

                if sh.lang.needs_line_check(sh, states) {
                    states.get_mut::<LineContents>().lines += states
                        .get::<LineContents>()
                        .cb
                        .as_str()
                        .into_owned()
                        .as_str();
                    states.get_mut::<LineContents>().lines += "\n";
                    states.get_mut::<LineContents>().cb.clear();

                    return Ok(false);
                }

                return Ok(true);
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                // if current input is empty exit the shell, otherwise treat it as enter
                if states.get::<LineContents>().cb.is_empty() {
                    // TODO maybe unify exiting the shell
                    let _ = disable_raw_mode(); // TODO this is temp fix, should be more graceful way of
                                                // handling cleanup code
                    std::process::exit(0);
                } else {
                    states.get_mut::<Box<dyn BufferHistory>>().clear();
                    self.painter.newline()?;
                    return Ok(true);
                }
            },

            _ => (),
        };

        Ok(false)
    }

    /// returns a bool whether input should still be handled
    pub fn expand(&mut self, states: &mut States, event: &Event) -> anyhow::Result<bool> {
        if !states.get::<Snippets>().should_expand(event) {
            return Ok(true);
        }
        //find current word

        let cur_line = states.get::<LineContents>().cb.as_str().to_string();
        let mut words = cur_line.split(' ').collect::<Vec<_>>();
        let mut char_offset = 0;
        //cursor is positioned just after the last typed character
        let index_before_cursor = states.get::<LineContents>().cb.cursor();
        let mut cur_word_index = None;
        for (i, word) in words.iter().enumerate() {
            // Determine the start and end indices of the current word
            let start_index = char_offset;
            let end_index = char_offset + word.len();

            // Check if the cursor index falls within the current word
            if index_before_cursor >= start_index && index_before_cursor <= end_index {
                cur_word_index = Some(i);
            }

            // Update the character offset to account for the current word and whitespace
            char_offset = end_index + 1; // Add 1 for the space between words
        }

        if let Some(c) = cur_word_index {
            if let Some(expanded) = states.get::<Snippets>().get(&words[c].to_string()) {
                //check if we're we're expanding the first word
                if expanded.position == Position::Command {
                    if c != 0 {
                        return Ok(true);
                    }
                }
                words[c] = expanded.value.as_str();

                states.get_mut::<LineContents>().cb.clear();
                //cursor automatically positioned at end
                states
                    .get_mut::<LineContents>()
                    .cb
                    .insert(Location::Cursor(), words.join(" ").as_str())?;
                return Ok(false);
            }
        }
        return Ok(true);
    }

    fn handle_insert_keys(
        &mut self,
        sh: &mut Shell,
        states: &mut States,
        event: Event,
    ) -> anyhow::Result<()> {
        if !self.expand(states, &event)? {
            return Ok(());
        }

        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                self.populate_completions(states)?;
                let mut menu = states.get_mut::<DefaultMenuState>();
                menu.activate();

                let completion_len = menu.items().len();

                // no-op if no completions
                if completion_len == 0 {
                    menu.disactivate();
                    return Ok(());
                }
                // if completions only has one entry, automatically select it
                if completion_len == 1 {
                    // TODO stupid ownership stuff
                    let item = menu.items().get(0).map(|x| (*x).clone()).unwrap();
                    self.accept_completion(states, item.1)?;
                    menu.disactivate();
                    return Ok(());
                }

                // TODO make this feature toggable
                // TODO this is broken
                // Automatically accept the common prefix
                /*
                let completions: Vec<&str> = self
                    .menu
                    .items()
                    .iter()
                    .map(|(preview, _)| preview.as_str())
                    .collect();
                let prefix = longest_common_prefix(completions);
                self.accept_completion(
                    states,
                    Completion {
                        add_space: false,
                        display: None,
                        completion: prefix.clone(),
                        replace_method: ReplaceMethod::Append,
                    },
                )?;

                // recompute completions with prefix stripped
                // TODO this code is horrifying
                let items = self.menu.items();
                let new_items = items
                    .iter()
                    .map(|(preview, complete)| {
                        let mut complete = complete.clone();
                        complete.completion = complete.completion[prefix.len()..].to_string();
                        (preview.clone(), complete)
                    })
                    .collect();
                self.menu.set_items(new_items);
                */

                menu.activate();
            },
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if states.get::<LineContents>().cb.cursor() > 0 {
                    states
                        .get_mut::<LineContents>()
                        .cb
                        .move_cursor(Location::Before())?;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if states.get::<LineContents>().cb.cursor() < states.get::<LineContents>().cb.len()
                {
                    states
                        .get_mut::<LineContents>()
                        .cb
                        .move_cursor(Location::After())?;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                self.history_down(sh, states)?;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                self.history_up(sh, states)?;
            },
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                self.to_normal_mode(sh, states)?;
                states
                    .get_mut::<Box<dyn BufferHistory>>()
                    .add(&states.get::<LineContents>().cb);
            },
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                if !states.get::<LineContents>().cb.is_empty()
                    && states.get::<LineContents>().cb.cursor() != 0
                {
                    states
                        .get_mut::<LineContents>()
                        .cb
                        .delete(Location::Before(), Location::Cursor())?;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                if !states.get::<LineContents>().cb.is_empty()
                    && states.get::<LineContents>().cb.cursor() != 0
                {
                    let start = states
                        .get::<LineContents>()
                        .cb
                        .motion_to_loc(Motion::BackWord)?;
                    states
                        .get_mut::<LineContents>()
                        .cb
                        .delete(start, Location::Cursor())?;
                }
            },

            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                states
                    .get_mut::<LineContents>()
                    .cb
                    .move_cursor(Location::Front())?;
            },

            Event::Key(KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                let back = Location::Back(&states.get::<LineContents>().cb);
                states.get_mut::<LineContents>().cb.move_cursor(back)?;
            },

            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                states
                    .get_mut::<LineContents>()
                    .cb
                    .insert(Location::Cursor(), &c.to_string())?;
            },
            _ => {},
        };
        Ok(())
    }

    fn handle_normal_keys(
        &mut self,
        sh: &mut Shell,
        states: &mut States,
        event: Event,
    ) -> anyhow::Result<()> {
        // TODO write better system toString support key combinations
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => {
                self.normal_keys.clear();
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                self.normal_keys.push(c);

                if let Ok(Command { repeat, action }) = Parser::default().parse(&self.normal_keys) {
                    for _ in 0..repeat {
                        // special cases (possibly consulidate with execute_vi somehow)

                        let mode = states
                            .get_mut::<LineContents>()
                            .cb
                            .execute_vi(action.clone());
                        if let Ok(m) = mode {
                            if m != *states.get::<LineMode>() {
                                match m {
                                    LineMode::Insert => self.to_insert_mode(sh, states)?,
                                    LineMode::Normal => self.to_normal_mode(sh, states)?,
                                };
                            }
                        }
                        match action {
                            Action::Undo => states
                                .get_mut::<Box<dyn BufferHistory>>()
                                .prev(&mut states.get_mut::<LineContents>().cb),

                            Action::Redo => states
                                .get_mut::<Box<dyn BufferHistory>>()
                                .next(&mut states.get_mut::<LineContents>().cb),
                            Action::Move(motion) => match motion {
                                Motion::Up => self.history_up(sh, states)?,
                                Motion::Down => self.history_down(sh, states)?,
                                _ => {},
                            },
                            Action::Editor => {
                                // TODO should this just use the env var? or should shrs have
                                // dedicated config?

                                // If EDITOR command is not set just display some sort of warning
                                // and move on
                                let Ok(editor) = std::env::var("EDITOR") else {
                                    return Ok(());
                                };

                                let mut tempbuf = tempfile::NamedTempFile::new().unwrap();

                                // write contexts of line to file
                                tempbuf
                                    .write_all(states.get::<LineContents>().cb.as_str().as_bytes())
                                    .unwrap();

                                // TODO should use shrs_job for this?
                                // TODO configure the command used
                                let mut child = std::process::Command::new(editor)
                                    .arg(tempbuf.path())
                                    .spawn()
                                    .unwrap();

                                child.wait().unwrap();

                                // read update file contexts back to line
                                let mut new_contents = String::new();
                                tempbuf.rewind().unwrap();
                                tempbuf.read_to_string(&mut new_contents).unwrap();

                                // strip last newline
                                // TODO this is very platform and editor dependent
                                let trimmed = new_contents.trim_end_matches("\n");

                                states.get_mut::<LineContents>().cb.clear();
                                states
                                    .get_mut::<LineContents>()
                                    .cb
                                    .insert(Location::Cursor(), trimmed)
                                    .unwrap();

                                // TODO should auto run the command?

                                tempbuf.close().unwrap();
                            },
                            _ => {
                                states
                                    .get_mut::<Box<dyn BufferHistory>>()
                                    .add(&states.get::<LineContents>().cb);
                            },
                        }
                    }

                    self.normal_keys.clear();
                }
            },
            _ => {},
        }
        Ok(())
    }

    // recalculate the current completions
    fn populate_completions(&mut self, states: &mut States) -> anyhow::Result<()> {
        // TODO IFS
        let mut line_contents = states.get::<LineContents>();
        let cursor = line_contents.cb.cursor();

        let args = line_contents
            .cb
            .slice(..cursor)
            .as_str()
            .unwrap()
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        *states.get_mut::<CurrentWord>() =
            CurrentWord(args.last().unwrap_or(&String::new()).clone());

        let comp_states = CompletionCtx::new(args);

        let completions = states.get::<Box<dyn Completer>>().complete(&comp_states);
        let completions = completions.iter().collect::<Vec<_>>();

        let menuitems = completions
            .iter()
            .map(|c| (c.display(), (*c).clone()))
            .collect::<Vec<_>>();
        states.get_mut::<DefaultMenuState>().set_items(menuitems);

        Ok(())
    }

    // replace word at cursor with accepted word (used in automcompletion)
    fn accept_completion(&mut self, states: &States, completion: Completion) -> anyhow::Result<()> {
        // first remove current word
        // TODO could implement a delete_before
        // TODO make use of ReplaceMethod
        match completion.replace_method {
            ReplaceMethod::Append => {
                // no-op
            },
            ReplaceMethod::Replace => {
                let cur_word_len = states.get::<CurrentWord>().len() as isize;
                states
                    .get_mut::<LineContents>()
                    .cb
                    .move_cursor(Location::Rel(-cur_word_len))?;
                let cur_word_width =
                    unicode_width::UnicodeWidthStr::width(states.get::<CurrentWord>().as_str());
                states
                    .get_mut::<LineContents>()
                    .cb
                    .delete(Location::Cursor(), Location::Rel(cur_word_width as isize))?;
                states.get_mut::<CurrentWord>().clear();
            },
        }

        // then replace with the completion word
        states
            .get_mut::<LineContents>()
            .cb
            .insert(Location::Cursor(), &completion.accept())?;

        Ok(())
    }

    fn history_up(&mut self, sh: &mut Shell, states: &mut States) -> anyhow::Result<()> {
        let cur_line = states.get::<LineContents>().cb.slice(..).to_string();
        // save current prompt
        if HistoryInd::Prompt == *states.get::<HistoryInd>() {
            *states.get_mut::<SavedLine>() = SavedLine(cur_line);
        }

        let next_ind = states.get::<HistoryInd>().up(sh.history.len(sh, states));
        *states.get_mut::<HistoryInd>() = next_ind;
        self.update_history(sh, states)?;

        Ok(())
    }

    fn history_down(&mut self, sh: &mut Shell, states: &mut States) -> anyhow::Result<()> {
        let next_ind = states.get::<HistoryInd>().down();
        *states.get_mut::<HistoryInd>() = next_ind;
        self.update_history(sh, states)?;

        Ok(())
    }

    fn update_history(&mut self, sh: &mut Shell, states: &mut States) -> anyhow::Result<()> {
        match *states.get::<HistoryInd>() {
            // restore saved line
            HistoryInd::Prompt => {
                states.get_mut::<LineContents>().cb.clear();
                states
                    .get_mut::<LineContents>()
                    .cb
                    .insert(Location::Cursor(), states.get::<SavedLine>().as_str())?;
            },
            // fill prompt with history element
            HistoryInd::Line(i) => {
                let history_item = sh.history.get(sh, states, i).unwrap();
                states.get_mut::<LineContents>().cb.clear();

                states
                    .get_mut::<LineContents>()
                    .cb
                    .insert(Location::Cursor(), history_item.as_str())?;
            },
        }
        Ok(())
    }

    fn to_normal_mode(&self, sh: &mut Shell, states: &mut States) -> anyhow::Result<()> {
        *states.get_mut::<LineMode>() = LineMode::Normal;

        let hook_states = LineModeSwitchCtx {
            line_mode: LineMode::Normal,
        };
        sh.run_hooks(states, hook_states)?;
        Ok(())
    }

    fn to_insert_mode(&self, sh: &mut Shell, states: &mut States) -> anyhow::Result<()> {
        *states.get_mut::<LineMode>() = LineMode::Insert;

        let hook_states = LineModeSwitchCtx {
            line_mode: LineMode::Insert,
        };
        sh.run_hooks(states, hook_states)?;

        Ok(())
    }
}
