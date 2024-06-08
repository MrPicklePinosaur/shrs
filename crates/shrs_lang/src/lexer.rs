//! Custom lexer

// heavily inspired by https://github.com/nixpulvis/oursh/blob/develop/src/program/posix/lex.rs

use std::str::CharIndices;

use lazy_static::lazy_static;
use thiserror::Error;

lazy_static! {
    pub static ref RESERVED_WORDS: Vec<&'static str> = vec![
        "!", "{", "}", "case", "do", "done", "elif", "else", "esac", "fi", "for", "if", "in",
        "then", "until", "while"
    ];
}

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token<'input> {
    NEWLINE,
    SEMI,
    AMP,
    PIPE,
    BACKTICK,
    EQUAL,
    BACKSLASH,
    SINGLEQUOTE,
    DOUBLEQUOTE,
    LESS,
    GREAT,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    BANG,

    AND_IF,
    OR_IF,
    DSEMI,

    DLESS,
    DGREAT,
    LESSAND,
    GREATAND,
    LESSGREAT,
    DLESSDASH,
    CLOBBER,

    IF,
    THEN,
    ELSE,
    ELIF,
    FI,
    DO,
    DONE,

    CASE,
    ESAC,
    WHILE,
    UNTIL,
    FOR,
    IN,

    WORD(&'input str),
    ASSIGNMENT_WORD(&'input str),
    FNAME(&'input str),
    NAME(&'input str),
    IO_NUMBER(&'input str),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum Error {
    #[error("unrecognized character {1} in range {0}:{2}")]
    UnrecognizedChar(usize, char, usize),
}

// TODO could technically make EOF a token so we don't need to do Result<Option> shinengans
#[derive(Clone)]
pub struct Lexer<'input> {
    input: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char, usize)>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();
        let next = chars.next();
        let lookahead = next.map(|n| (n.0, n.1, n.0 + n.1.len_utf8()));
        Lexer {
            input,
            chars,
            lookahead,
        }
    }

    pub fn input(&self) -> &'input str {
        self.input
    }

    fn advance(&mut self) -> Option<(usize, char, usize)> {
        match self.lookahead {
            Some((start, ch, end)) => {
                let next = self.chars.next();
                self.lookahead = next.map(|n| (n.0, n.1, n.0 + n.1.len_utf8()));
                Some((start, ch, end))
            },
            // EOF case
            None => None,
        }
    }

    fn keyword(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<(usize, Token<'input>, usize), Error> {
        let (word, end) = self.take_until(start, end, |ch| !is_word_continue(ch));
        let token = match word {
            "if" => Token::IF,
            "then" => Token::THEN,
            "else" => Token::ELSE,
            "elif" => Token::ELIF,
            "fi" => Token::FI,
            "do" => Token::DO,
            "done" => Token::DONE,
            "case" => Token::CASE,
            "esac" => Token::ESAC,
            "while" => Token::WHILE,
            "until" => Token::UNTIL,
            "for" => Token::FOR,
            "in" => Token::IN,
            word => Token::WORD(word),
        };
        Ok((start, token, end))
    }

    // TODO escape characters
    fn single_quote(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<(usize, Token<'input>, usize), Error> {
        let (_, end) = self.take_until(start + 1, end, |ch| ch == '\'');
        self.advance();
        Ok((start, Token::WORD(&self.input[start + 1..end]), end))
    }

    fn double_quote(
        &mut self,
        start: usize,
        end: usize,
    ) -> Result<(usize, Token<'input>, usize), Error> {
        let (_, end) = self.take_until(start + 1, end, |ch| ch == '"');
        self.advance();
        Ok((start, Token::WORD(&self.input[start + 1..end]), end))
    }

    // utils for reading until condition is met
    fn take_until<F>(
        &mut self,
        start: usize,
        mut end: usize,
        mut terminate: F,
    ) -> (&'input str, usize)
    where
        F: FnMut(char) -> bool,
    {
        while let Some((_, ch, _)) = self.lookahead {
            if terminate(ch) {
                return (&self.input[start..end], end);
            } else if let Some((_, _, e)) = self.advance() {
                end = e;
            }
        }
        (&self.input[start..end], end)
    }
    fn take_until_inclusive<F>(
        &mut self,
        start: usize,
        mut end: usize,
        mut terminate: F,
    ) -> (&'input str, usize)
    where
        F: FnMut(char) -> bool,
    {
        while let Some((_, ch, _)) = self.lookahead {
            if terminate(ch) {
                return (&self.input[start..=end], end + 1);
            } else if let Some((_, _, e)) = self.advance() {
                end = e;
            }
        }
        (&self.input[start..end], end)
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, Error>;

    // TODO create proc macro to generate all this?
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, ch, end)) = self.advance() {
            // TODO see if this could be generated with macro
            let token = match ch {
                '\n' => Some(Ok((start, Token::NEWLINE, end))),
                ';' => match self.lookahead {
                    Some((_, ';', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::DSEMI, new_end)))
                    },
                    _ => Some(Ok((start, Token::SEMI, end))),
                },
                '&' => match self.lookahead {
                    Some((_, '&', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::AND_IF, new_end)))
                    },
                    _ => Some(Ok((start, Token::AMP, end))),
                },
                '|' => match self.lookahead {
                    Some((_, '|', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::OR_IF, new_end)))
                    },
                    _ => Some(Ok((start, Token::PIPE, end))),
                },
                '`' => Some(Ok((start, Token::BACKTICK, end))),
                '=' => Some(Ok((start, Token::EQUAL, end))),
                '\\' => Some(Ok((start, Token::BACKSLASH, end))),
                '<' => match self.lookahead {
                    // TODO current doesn't support <<-
                    Some((_, '<', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::DLESS, new_end)))
                    },
                    Some((_, '&', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::LESSAND, new_end)))
                    },
                    Some((_, '>', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::LESSGREAT, new_end)))
                    },
                    _ => Some(Ok((start, Token::LESS, end))),
                },
                '>' => match self.lookahead {
                    Some((_, '>', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::DGREAT, new_end)))
                    },
                    Some((_, '&', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::GREATAND, new_end)))
                    },
                    Some((_, '|', new_end)) => {
                        self.advance();
                        Some(Ok((start, Token::CLOBBER, new_end)))
                    },
                    _ => Some(Ok((start, Token::GREAT, end))),
                },

                '(' => Some(Ok((start, Token::LPAREN, end))),
                ')' => Some(Ok((start, Token::RPAREN, end))),
                '{' => Some(Ok((start, Token::LBRACE, end))),
                '}' => Some(Ok((start, Token::RBRACE, end))),
                '!' => Some(Ok((start, Token::BANG, end))),
                '\'' => Some(self.single_quote(start, end)),
                '"' => Some(self.double_quote(start, end)),
                ch if is_word_start(ch) => Some(self.keyword(start, end)),
                ch if ch.is_whitespace() => continue,
                ch => return Some(Err(Error::UnrecognizedChar(start, ch, end))),
            };
            return token;
        }
        None
    }
}

/// predicate that detects when a word starts (non whitespace, non control character)
fn is_word_start(ch: char) -> bool {
    match ch {
        '\u{007F}' | '\u{0000}'..='\u{001F}' | '\u{0080}'..='\u{009F}' => false,
        _ => is_word_continue(ch),
    }
}

/// predicate for when to keep reading word token
fn is_word_continue(ch: char) -> bool {
    match ch {
        ';' | ')' | '(' | '`' | '!' | '\'' | '"' | '>' | '<' | '&' | '|' | '{' | '}' => false,
        _ => !ch.is_whitespace(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn single_quote() {
        let mut lexer = Lexer::new("'hello world'");
        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::WORD("'hello world'"), 13)))
        );
    }

    #[test]
    fn keywords() {
        let mut lexer = Lexer::new("case");
        assert_eq!(lexer.next(), Some(Ok((0, Token::CASE, 4))));
    }
}
