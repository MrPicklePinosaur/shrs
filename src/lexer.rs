use std::str::CharIndices;

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

#[allow(non_camel_case_types)]
pub enum Token<'input> {
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

    LBRACE,
    RBRACE,
    BANG,
    IN,

    WORD(&'input str),
    ASSIGNMENT_WORD(&'input str),
    NAME(&'input str),
    IO_NUMBER(&'input str),
}

pub enum Error {}

pub struct Lexer<'input> {
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            chars: input.char_indices(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<'input>, usize, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
