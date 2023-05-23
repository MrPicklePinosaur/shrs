use std::{
    collections::HashMap,
    ops::{Range, RangeBounds},
};

use crossterm::style::{Color, ContentStyle, StyledContent};
use shrs_lang::{ast, Lexer, Parser, Token, RESERVED_WORDS};

use crate::painter::StyledBuf;

pub trait Highlighter {
    fn highlight(&self, buf: &str) -> StyledBuf;
}

/// Simple highlighter that colors the entire line one color
#[derive(Default)]
pub struct DefaultHighlighter {
    pub style: ContentStyle,
}

impl Highlighter for DefaultHighlighter {
    fn highlight(&self, buf: &str) -> StyledBuf {
        let mut styled_buf = StyledBuf::new();

        styled_buf.push(StyledContent::new(
            ContentStyle {
                foreground_color: Some(Color::Green),
                ..Default::default()
            },
            buf.to_string(),
        ));

        styled_buf
    }
}

pub type RuleFn = fn(&Token) -> bool;

pub struct SyntaxTheme {
    pub command: ContentStyle,
    pub auto: ContentStyle, // path: ContentStyle
    // RuleFn returns true if style should be applied to token
    pub style_rules: Vec<(RuleFn, ContentStyle)>,
}

impl SyntaxTheme {
    fn new(command: ContentStyle, auto: ContentStyle) -> Self {
        Self {
            command,
            auto,
            style_rules: vec![],
        }
    }
    pub fn push_rule(&mut self, rule: RuleFn, style: ContentStyle) {
        self.style_rules.push((rule, style));
    }
}

impl Default for SyntaxTheme {
    fn default() -> Self {
        let mut rules = vec![];
        let is_reserved: RuleFn = |t: &Token| -> bool {
            match t {
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
                | Token::IN => true,
                _ => false,
            }
        };
        let is_string: RuleFn = |t: &Token| -> bool {
            if let Token::WORD(w) = t {
                return w.starts_with('\'') || w.starts_with('\"');
            }
            false
        };

        rules.push((
            is_reserved,
            ContentStyle {
                foreground_color: Some(Color::Yellow),
                ..Default::default()
            },
        ));
        rules.push((
            is_string,
            ContentStyle {
                foreground_color: Some(Color::Green),
                ..Default::default()
            },
        ));

        Self {
            command: ContentStyle {
                foreground_color: Some(Color::Blue),
                ..Default::default()
            },

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
    fn highlight(&self, buf: &str) -> StyledBuf {
        let mut last_index = 0;
        let mut is_cmd = true;
        let mut style = self.theme.auto;

        let lexer = Lexer::new(buf);

        let mut styled_buf = StyledBuf::new();
        for t in lexer {
            style = self.theme.auto;

            if let Ok(token) = t {
                match token.1.clone() {
                    Token::WORD(_) => {
                        if is_cmd {
                            style = self.theme.command;
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
                for (style_rule, s) in self.theme.style_rules.iter() {
                    if style_rule(&token.1) {
                        style = s.clone();
                        break;
                    }
                }

                // pushes spaces before token to end of token
                styled_buf.push(StyledContent::new(
                    style,
                    buf[last_index..token.2].to_string(),
                ));
                last_index = token.2
            }
        }
        // pushes remaining content after last token
        styled_buf.push(StyledContent::new(
            style,
            buf[last_index..buf.len()].to_string(),
        ));

        styled_buf
    }
}
