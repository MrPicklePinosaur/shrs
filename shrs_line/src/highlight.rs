use std::{collections::HashMap, usize};

use crossterm::style::{Color, ContentStyle, StyledContent};
use shrs_lang::{ast, Lexer, Parser, Token, RESERVED_WORDS};

use crate::painter::StyledBuf;

pub trait Highlighter {
    fn highlight(&self, buf: &str, begin: usize) -> StyledBuf;
}

/// Simple highlighter that colors the entire line one color
#[derive(Default)]
pub struct DefaultHighlighter {
    pub style: ContentStyle,
}

impl Highlighter for DefaultHighlighter {
    fn highlight(&self, buf: &str, begin: usize) -> StyledBuf {
        let mut styled_buf = StyledBuf::empty();

        styled_buf.push(
            &buf[begin..],
            ContentStyle {
                foreground_color: Some(Color::Green),
                ..Default::default()
            },
        );

        styled_buf
    }
}

pub type RuleFn = fn(&str) -> HashMap<usize, ContentStyle>;

pub struct SyntaxTheme {
    pub auto: ContentStyle, // path: ContentStyle
    // RuleFn returns iterator of charindex and style
    pub style_rules: Vec<RuleFn>,
}

impl SyntaxTheme {
    fn new(auto: ContentStyle) -> Self {
        Self {
            auto,
            style_rules: vec![],
        }
    }
    pub fn push_rule(&mut self, rule: RuleFn, style: ContentStyle) {
        self.style_rules.push(rule);
    }
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        let mut rules = vec![];

        let shrs_rule: RuleFn = |buf: &str| -> HashMap<usize, ContentStyle> {
            let cmd_style = ContentStyle {
                foreground_color: Some(Color::Blue),
                ..Default::default()
            };
            let string_style = ContentStyle {
                foreground_color: Some(Color::Green),
                ..Default::default()
            };
            let reserved_style = ContentStyle {
                foreground_color: Some(Color::Yellow),
                ..Default::default()
            };

            let mut c_style: HashMap<usize, ContentStyle> = HashMap::new();
            let mut range_insert = |start: usize, end: usize, style: ContentStyle| {
                (start..end).into_iter().for_each(|u| {
                    c_style.insert(u, style);
                })
            };

            let lexer = Lexer::new(buf);
            let mut is_cmd = true;
            for t in lexer {
                if let Ok(token) = t {
                    match token.1.clone() {
                        Token::WORD(_) => {
                            if is_cmd {
                                range_insert(token.0, token.2, cmd_style);
                                is_cmd = false;
                            }
                        },
                        //Tokens that make next word command
                        Token::IF
                        | Token::THEN
                        | Token::ELSE
                        | Token::ELIF
                        | Token::DO
                        | Token::CASE
                        | Token::AND_IF
                        | Token::OR_IF
                        | Token::SEMI
                        | Token::DSEMI
                        | Token::AMP
                        | Token::PIPE => {
                            is_cmd = true;
                        },
                        _ => (),
                    }
                    match token.1 {
                        Token::IF
                        | Token::ELSE
                        | Token::FI
                        | Token::THEN
                        | Token::ELIF
                        | Token::DO
                        | Token::DONE
                        | Token::CASE
                        | Token::ESAC
                        | Token::WHILE
                        | Token::UNTIL
                        | Token::FOR
                        | Token::IN => {
                            range_insert(token.0, token.2, reserved_style);
                        },
                        _ => (),
                    }
                    if let Token::WORD(w) = token.1 {
                        if w.starts_with('\'') || w.starts_with('\"') {
                            range_insert(token.0, token.2, string_style);
                        }
                    }
                }
            }
            c_style
        };
        rules.push(shrs_rule);
        Self {
            auto: ContentStyle::default(),
            style_rules: rules,
        }
    }
}

pub struct SyntaxHighlighter {
    theme: SyntaxTheme,
}

impl SyntaxHighlighter {
    pub fn new(theme: SyntaxTheme) -> Self {
        SyntaxHighlighter { theme }
    }
}

impl Highlighter for SyntaxHighlighter {
    fn highlight(&self, buf: &str, begin: usize) -> StyledBuf {
        let mut styled_buf = StyledBuf::new(&buf[begin..], self.theme.auto);

        for style_rule in self.theme.style_rules.iter() {
            styled_buf.change_style(style_rule(buf), begin);
        }

        styled_buf
    }
}
