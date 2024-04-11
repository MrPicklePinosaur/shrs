use crossterm::style::ContentStyle;
use rustpython_parser::{lexer::lex, Mode, Tok};
use shrs::{
    crossterm::{Color, Stylize},
    readline::SyntaxTheme,
};

pub struct PythonSytaxTheme {
    pub name_style: ContentStyle,
    pub number_style: ContentStyle,
    pub string_style: ContentStyle,
    pub keyword_style: ContentStyle,
    pub operator_style: ContentStyle,
    pub punctuation_style: ContentStyle,
}
impl PythonSytaxTheme {
    pub fn new() -> Self {
        PythonSytaxTheme {
            name_style: ContentStyle::new().white(),
            number_style: ContentStyle::new().dark_green(),
            string_style: ContentStyle::new().green(),
            keyword_style: ContentStyle::new().yellow(),
            operator_style: ContentStyle::new().dark_blue(),
            punctuation_style: ContentStyle::new().blue(),
        }
    }
    pub fn match_token(&self, token: Tok) -> ContentStyle {
        match token {
            Tok::Name { .. } => self.name_style,
            Tok::Int { .. } | Tok::Float { .. } | Tok::Complex { .. } => self.number_style,
            Tok::String { .. } => self.string_style,
            Tok::Lpar | Tok::Rpar | Tok::Lsqb | Tok::Rsqb | Tok::Colon | Tok::Comma | Tok::Semi => {
                self.punctuation_style
            },
            Tok::Plus
            | Tok::Minus
            | Tok::Star
            | Tok::Slash
            | Tok::Vbar
            | Tok::Amper
            | Tok::Less
            | Tok::Greater
            | Tok::Equal
            | Tok::Dot
            | Tok::Percent
            | Tok::Lbrace
            | Tok::Rbrace
            | Tok::EqEqual
            | Tok::NotEqual
            | Tok::LessEqual
            | Tok::GreaterEqual
            | Tok::Tilde
            | Tok::CircumFlex
            | Tok::LeftShift
            | Tok::RightShift
            | Tok::DoubleStar
            | Tok::DoubleStarEqual
            | Tok::PlusEqual
            | Tok::MinusEqual
            | Tok::StarEqual
            | Tok::SlashEqual
            | Tok::PercentEqual
            | Tok::AmperEqual
            | Tok::VbarEqual
            | Tok::CircumflexEqual
            | Tok::LeftShiftEqual
            | Tok::RightShiftEqual
            | Tok::DoubleSlash
            | Tok::DoubleSlashEqual
            | Tok::ColonEqual
            | Tok::At
            | Tok::AtEqual
            | Tok::Rarrow
            | Tok::Ellipsis => self.operator_style,
            Tok::False
            | Tok::None
            | Tok::True
            | Tok::And
            | Tok::As
            | Tok::Assert
            | Tok::Async
            | Tok::Await
            | Tok::Break
            | Tok::Class
            | Tok::Continue
            | Tok::Def
            | Tok::Del
            | Tok::Elif
            | Tok::Else
            | Tok::Except
            | Tok::Finally
            | Tok::For
            | Tok::From
            | Tok::Global
            | Tok::If
            | Tok::Import
            | Tok::In
            | Tok::Is
            | Tok::Lambda
            | Tok::Nonlocal
            | Tok::Not
            | Tok::Or
            | Tok::Pass
            | Tok::Raise
            | Tok::Return
            | Tok::Try
            | Tok::While
            | Tok::Match
            | Tok::Type
            | Tok::Case
            | Tok::With
            | Tok::Yield => self.keyword_style,
            _ => ContentStyle::new(),
        }
    }
}
impl SyntaxTheme for PythonSytaxTheme {
    fn apply(&self, buf: &mut shrs::prelude::styled_buf::StyledBuf) {
        // yields tokens, if error will continue yielding infinitely
        let content = buf.content.to_owned();
        let tokens = lex(content.as_str(), Mode::Expression);

        for t in tokens {
            if let Ok((token, range)) = t {
                buf.apply_style_in_range(
                    range.start().to_usize()..range.end().to_usize(),
                    self.match_token(token),
                );
            } else {
                break;
            }
        }
    }
}
