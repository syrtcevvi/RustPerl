use std::ops::Range;

use enum_as_inner::EnumAsInner;
use logos::Logos;

use crate::interpreter::error::LexingError;

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Logos, EnumAsInner)]
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

    #[regex(r"(\$|@|%)?[a-zA-Z_][a-zA-Z0-9_]*", ident)]
    Ident(Ident<'src>),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ident<'src> {
    Bare(&'src str),
    Scalar(&'src str),
    Array(&'src str),
    Hash(&'src str),
}

fn ident<'src>(lex: &mut logos::Lexer<'src, TokenKind<'src>>) -> Result<Ident<'src>, LexingError> {
    match &lex.slice()[0..1] {
        "$" => Ok(Ident::Scalar(&lex.slice()[1..])),
        "@" => Ok(Ident::Array(&lex.slice()[1..])),
        "%" => Ok(Ident::Hash(&lex.slice()[1..])),
        _ => Ok(Ident::Bare(lex.slice())),
    }
}
