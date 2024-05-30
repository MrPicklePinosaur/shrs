//! Syntax highlighting

use std::marker::PhantomData;

use anyhow::Result;
use crossterm::style::{Color, ContentStyle};
use shrs_lang::{Lexer, Token};
use shrs_utils::StyledBuf;

use super::super::prelude::Param;
use crate::prelude::{Shell, States};

/// Simple highlighter that colors the entire line one color
#[derive(Default)]
pub struct DefaultHighlighter {
    pub style: ContentStyle,
}

impl Highlighter for DefaultHighlighter {
    fn highlight(&self, _sh: &Shell, _ctx: &States, buf: &String) -> Result<StyledBuf> {
        let mut styled_buf = StyledBuf::empty();

        styled_buf.push(
            &buf,
            ContentStyle {
                foreground_color: Some(Color::Green),
                ..Default::default()
            },
        );

        Ok(styled_buf)
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
    fn highlight(&self, _sh: &Shell, _ctx: &States, buf: &String) -> Result<StyledBuf> {
        let mut styled_buf = StyledBuf::new(&buf).style(self.auto);

        for syntax_theme in self.syntax_themes.iter() {
            syntax_theme.apply(&mut styled_buf);
        }

        Ok(styled_buf)
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
/// Implement this trait to define your own highlighter command
pub trait Highlighter {
    /// highlight buf
    fn highlight(&self, sh: &Shell, states: &States, buf: &String) -> Result<StyledBuf>;
}

pub trait IntoHighlighter<Input> {
    type Highlighter: Highlighter;
    fn into_highlighter(self) -> Self::Highlighter;
}
pub struct FunctionHighlighter<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}
impl<F> Highlighter for FunctionHighlighter<(Shell, String), F>
where
    for<'a, 'b> &'a F: Fn(&Shell, &String) -> Result<StyledBuf>,
{
    fn highlight(&self, sh: &Shell, _ctx: &States, buf: &String) -> Result<StyledBuf> {
        fn call_inner(
            f: impl Fn(&Shell, &String) -> Result<StyledBuf>,
            sh: &Shell,
            buf: &String,
        ) -> Result<StyledBuf> {
            f(&sh, &buf)
        }

        call_inner(&self.f, sh, &buf)
    }
}

macro_rules! impl_highlighter {
    (
        $($params:ident),*
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused)]
        impl<F, $($params: Param),+> Highlighter for FunctionHighlighter<($($params,)+), F>
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell,&String)->Result<StyledBuf> +
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell,&String )->Result<StyledBuf>
        {
            fn highlight(&self, sh: &Shell,states: &States, buf: &String)->Result<StyledBuf> {
                fn call_inner<$($params),+>(
                    f: impl Fn($($params),+,&Shell,&String)->Result<StyledBuf>,
                    $($params: $params),*
                    ,sh:&Shell,buf:&String
                ) -> Result<StyledBuf>{
                    f($($params),*,sh,buf)
                }

                $(
                    let $params = $params::retrieve(sh,states).unwrap();
                )+

                call_inner(&self.f, $($params),+,sh,&buf)
            }
        }
    }
}
impl<F> IntoHighlighter<()> for F
where
    for<'a, 'b> &'a F: Fn(&Shell, &String) -> Result<StyledBuf>,
{
    type Highlighter = FunctionHighlighter<(Shell, String), Self>;

    fn into_highlighter(self) -> Self::Highlighter {
        FunctionHighlighter {
            f: self,
            marker: Default::default(),
        }
    }
}
impl<H: Highlighter> IntoHighlighter<H> for H {
    type Highlighter = H;

    fn into_highlighter(self) -> Self::Highlighter {
        self
    }
}

macro_rules! impl_into_highlighter {
    (
        $($params:ident),+
    ) => {
        impl<F, $($params: Param),+> IntoHighlighter<($($params,)*)> for F
            where
                for<'a, 'b> &'a F:
                    Fn( $($params),+,&Shell,&String ) ->Result<StyledBuf>+
                    Fn( $(<$params as Param>::Item<'b>),+,&Shell,&String )->Result<StyledBuf>
        {
            type Highlighter = FunctionHighlighter<($($params,)+), Self>;

            fn into_highlighter(self) -> Self::Highlighter {
                FunctionHighlighter {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    }
}
all_the_tuples!(impl_highlighter, impl_into_highlighter);
