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
pub struct SyntaxTheme {
    command: ContentStyle,
    operator: ContentStyle,
    reserved: ContentStyle,
    auto: ContentStyle, // path: ContentStyle
}
impl Default for SyntaxTheme {
    fn default() -> Self {
        Self {
            command: ContentStyle {
                foreground_color: Some(Color::Blue),
                ..Default::default()
            },

            operator: ContentStyle::default(),
            reserved: ContentStyle {
                foreground_color: Some(Color::Yellow),
                ..Default::default()
            },
            auto: ContentStyle::default(),
        }
    }
}

pub struct SyntaxHighlighter {
    theme: SyntaxTheme,
}
impl SyntaxHighlighter {
    pub fn new() -> Self {
        SyntaxHighlighter {
            theme: SyntaxTheme::default(),
        }
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
                match token.1 {
                    Token::WORD(_) => {
                        if is_cmd {
                            style = self.theme.command;
                            is_cmd = false;
                        }
                    },
                    Token::LPAREN | Token::RPAREN | Token::RBRACE | Token::LBRACE => {
                        style = self.theme.operator;
                    },
                    //Tokens that are followed by command
                    Token::AND_IF | Token::OR_IF | Token::DSEMI | Token::SEMI => {
                        style = self.theme.operator;
                        is_cmd = true;
                    },
                    Token::IF
                    | Token::THEN
                    | Token::ELSE
                    | Token::ELIF
                    | Token::DO
                    | Token::CASE => {
                        style = self.theme.reserved;
                        is_cmd = true;
                    },
                    Token::FI | Token::ESAC => {
                        style = self.theme.reserved;
                    },
                    _ => (),
                }

                println!("{:?}", token);
                styled_buf.push(StyledContent::new(
                    style,
                    buf[last_index..token.2].to_string(),
                ));
                last_index = token.2
            }
        }
        styled_buf.push(StyledContent::new(
            style,
            buf[last_index..buf.len()].to_string(),
        ));

        styled_buf
    }
}
