//! Syntax highlighting

use std::{collections::HashMap, usize};

use crossterm::style::{Color, ContentStyle};
use shrs_lang::{Lexer, Token};
use shrs_utils::styled_buf::StyledBuf;

use super::line::LineStateBundle;

pub trait Highlighter {
    /// highlight buf, state allows access to additional info
    fn highlight(&self, state: &LineStateBundle, buf: &str) -> StyledBuf;
}

/// Simple highlighter that colors the entire line one color
#[derive(Default)]
pub struct DefaultHighlighter {
    pub style: ContentStyle,
}

impl Highlighter for DefaultHighlighter {
    fn highlight(&self, state: &LineStateBundle, buf: &str) -> StyledBuf {
        let mut styled_buf = StyledBuf::empty();

        styled_buf.push(
            &buf,
            ContentStyle {
                foreground_color: Some(Color::Green),
                ..Default::default()
            },
        );

        styled_buf
    }
}

/// trait that modifies a StyledBuf inplace and applies a theme to highlight the text
pub trait SyntaxTheme {
    fn apply(&self, buf: &mut StyledBuf);
}

pub struct SyntaxHighlighter {
    auto: ContentStyle,
    pub syntax_themes: Vec<Box<dyn SyntaxTheme>>,
}
impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self {
            auto: ContentStyle::default(),
            syntax_themes: vec![Box::new(ShrsTheme::default())],
        }
    }
}

impl SyntaxHighlighter {
    pub fn push_rule(&mut self, syntax_theme: Box<dyn SyntaxTheme>) {
        self.syntax_themes.push(syntax_theme);
    }

    pub fn new(auto: ContentStyle, themes: Vec<Box<dyn SyntaxTheme>>) -> Self {
        SyntaxHighlighter {
            auto,
            syntax_themes: themes,
        }
    }
}

impl Highlighter for SyntaxHighlighter {
    fn highlight(&self, state: &LineStateBundle, buf: &str) -> StyledBuf {
        let mut styled_buf = StyledBuf::new(&buf).style(self.auto);

        for syntax_theme in self.syntax_themes.iter() {
            syntax_theme.apply(&mut styled_buf);
        }

        styled_buf
    }
}
/// Implementation of a highlighter for the shrs language.
/// Utilizes the shrs parser to parse and highlight various tokens based on their type
pub struct ShrsTheme {
    cmd_style: ContentStyle,
    string_style: ContentStyle,
    reserved_style: ContentStyle,
}
impl Default for ShrsTheme {
    fn default() -> Self {
        ShrsTheme::new(
            ContentStyle {
                foreground_color: Some(Color::Blue),
                ..Default::default()
            },
            ContentStyle {
                foreground_color: Some(Color::Green),
                ..Default::default()
            },
            ContentStyle {
                foreground_color: Some(Color::Yellow),
                ..Default::default()
            },
        )
    }
}
impl ShrsTheme {
    pub fn new(
        cmd_style: ContentStyle,
        string_style: ContentStyle,
        reserved_style: ContentStyle,
    ) -> Self {
        ShrsTheme {
            cmd_style,
            string_style,
            reserved_style,
        }
    }
}
impl SyntaxTheme for ShrsTheme {
    fn apply(&self, buf: &mut StyledBuf) {
        let content = buf.content.clone();
        let lexer = Lexer::new(content.as_str());
        let mut is_cmd = true;
        for token in lexer.flatten() {
            match token.1.clone() {
                Token::WORD(_) => {
                    if is_cmd {
                        buf.apply_style_in_range(token.0..token.2, self.cmd_style);
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
                    buf.apply_style_in_range(token.0..token.2, self.reserved_style);
                },
                _ => (),
            }
            if let Token::WORD(w) = token.1 {
                if w.starts_with('\'') || w.starts_with('\"') {
                    buf.apply_style_in_range(token.0..token.2, self.string_style);
                }
            }
        }
    }
}
