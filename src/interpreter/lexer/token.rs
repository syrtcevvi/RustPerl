use std::ops::Range;

use logos::Logos;

use crate::interpreter::error::LexingError;

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Logos)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#[^\n]+")]
pub enum TokenKind<'src> {
    #[token("my")]
    My,

    #[regex(r"(?x) \( | \)")]
    RoundParen(&'src str),
    #[regex(r"(?x) \[ | \]")]
    SquareBracket(&'src str),
    #[regex(r"(?x) \{ | \}")]
    CurlyBrace(&'src str),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    #[token("=")]
    Equal,
    #[token(";")]
    Semicolon,
    #[token("$")]
    Dollar,

    #[regex("[a-zA-Z]+")]
    Ident(&'src str),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i64),
}
