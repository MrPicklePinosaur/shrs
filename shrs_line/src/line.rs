use std::{io::Write, time::Duration};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::{
    completion::{Completer, DefaultCompleter},
    cursor::{Cursor, DefaultCursor},
    history::{DefaultHistory, History},
    menu::{DefaultMenu, Menu},
    painter::Painter,
    prompt::Prompt,
};

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(setter(prefix = "with"))]
pub struct Line {
    #[builder(default = "Box::new(DefaultMenu::new())")]
    #[builder(setter(custom))]
    menu: Box<dyn Menu<MenuItem = String>>,

    #[builder(default = "Box::new(DefaultCompleter::new(vec![]))")]
    #[builder(setter(custom))]
    completer: Box<dyn Completer>,

    #[builder(default = "Box::new(DefaultHistory::new())")]
    #[builder(setter(custom))]
    history: Box<dyn History<HistoryItem = String>>,

    #[builder(default = "Box::new(DefaultCursor::default())")]
    #[builder(setter(custom))]
    cursor: Box<dyn Cursor>,
}

impl Default for Line {
    fn default() -> Self {
        LineBuilder::default().build().unwrap()
    }
}

// TODO none of the builder stuff is being autogenerated rn :()
impl LineBuilder {
    pub fn with_menu(mut self, menu: impl Menu<MenuItem = String> + 'static) -> Self {
        self.menu = Some(Box::new(menu));
        self
    }
    pub fn with_completer(mut self, completer: impl Completer + 'static) -> Self {
        self.completer = Some(Box::new(completer));
        self
    }
    pub fn with_history(mut self, history: impl History<HistoryItem = String> + 'static) -> Self {
        self.history = Some(Box::new(history));
        self
    }
    pub fn with_cursor(mut self, cursor: impl Cursor + 'static) -> Self {
        self.cursor = Some(Box::new(cursor));
        self
    }
}

impl Line {
    pub fn read_line<T: Prompt + ?Sized>(&mut self, prompt: impl AsRef<T>) -> String {
        // get line

        self.read_events(prompt).unwrap()
    }

    fn read_events<T: Prompt + ?Sized>(
        &mut self,
        prompt: impl AsRef<T>,
    ) -> crossterm::Result<String> {
        let mut buf: Vec<u8> = Vec::new();
        let mut ind: i32 = 0;

        let mut painter = Painter::new().unwrap();

        // TODO this is temp, find better way to store prefix of current word
        let mut current_word = String::new();

        // TODO dumping history index here for now
        let mut history_ind: i32 = -1;

        enable_raw_mode()?;

        painter.init(&prompt, &self.menu).unwrap();
        painter
            .paint(&prompt, &self.menu, "", ind as usize, &self.cursor)
            .unwrap();

        loop {
            if poll(Duration::from_millis(1000))? {
                let event = read()?;

                // handle menu events
                if self.menu.is_active() {
                    match event {
                        Event::Key(KeyEvent {
                            code: KeyCode::Enter,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                            if let Some(accepted) = self.menu.accept() {
                                accepted.chars().skip(current_word.len()).for_each(|c| {
                                    // TODO find way to insert multiple items in one operation
                                    buf.insert(ind as usize, c as u8);
                                    ind = (ind + 1).min(buf.len() as i32);
                                });
                            }
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
                            self.menu.previous();
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
                            self.menu.next();
                        },
                        _ => {},
                    }
                } else {
                    match event {
                        Event::Key(KeyEvent {
                            code: KeyCode::Enter,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                            painter.newline()?;
                            break;
                        },
                        Event::Key(KeyEvent {
                            code: KeyCode::Tab,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                            let res = std::str::from_utf8(buf.as_slice()).unwrap().to_string();

                            // TODO IFS
                            current_word = res.as_str()[..ind as usize]
                                .split(' ')
                                .last()
                                .unwrap_or("")
                                .to_string();
                            let completions = self.completer.complete(&current_word);
                            let owned = completions
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<_>>();
                            self.menu.set_items(owned);
                            self.menu.activate();
                        },
                        Event::Key(KeyEvent {
                            code: KeyCode::Left,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                            ind = (ind - 1).max(0);
                        },
                        Event::Key(KeyEvent {
                            code: KeyCode::Right,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                            ind = (ind + 1).min(buf.len() as i32);
                        },
                        Event::Key(KeyEvent {
                            code: KeyCode::Backspace,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                            if !buf.is_empty() {
                                ind = (ind - 1).max(0);
                                buf.remove(ind as usize);
                            }
                        },
                        Event::Key(KeyEvent {
                            code: KeyCode::Down,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                            history_ind = (history_ind - 1).max(0);
                            if let Some(history_item) = self.history.get(history_ind as usize) {
                                buf.clear();
                                let mut history_item =
                                    history_item.chars().map(|x| x as u8).collect::<Vec<_>>();
                                buf.append(&mut history_item);
                                ind = buf.len() as i32;
                            }
                        },
                        Event::Key(KeyEvent {
                            code: KeyCode::Up,
                            modifiers: KeyModifiers::NONE,
                            ..
                        }) => {
                            history_ind = if self.history.len() == 0 {
                                0
                            } else {
                                (history_ind + 1).min(self.history.len() as i32 - 1)
                            };
                            if let Some(history_item) = self.history.get(history_ind as usize) {
                                buf.clear();
                                let mut history_item =
                                    history_item.chars().map(|x| x as u8).collect::<Vec<_>>();
                                buf.append(&mut history_item);
                                ind = buf.len() as i32;
                            }
                        },
                        Event::Key(KeyEvent {
                            code: KeyCode::Char(c),
                            ..
                        }) => {
                            buf.insert(ind as usize, c as u8);
                            ind = (ind + 1).min(buf.len() as i32);
                        },
                        _ => {},
                    }
                }

                let res = std::str::from_utf8(buf.as_slice()).unwrap().to_string();

                painter
                    .paint(&prompt, &self.menu, &res, ind as usize, &self.cursor)
                    .unwrap();
            }
        }

        disable_raw_mode()?;

        let res = std::str::from_utf8(buf.as_slice()).unwrap().to_string();
        self.history.add(res.clone());
        Ok(res)
    }
}
