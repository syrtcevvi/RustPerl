use std::ops::Range;

use enum_as_inner::EnumAsInner;
use logos::Logos;

use crate::interpreter::error::LexingError;

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Logos, EnumAsInner, derive_more::Display)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#[^\n]+")]
pub enum TokenKind<'src> {
    #[token("if")]
    #[display("if")]
    If,
    #[token("elsif")]
    #[display("elsif")]
    Elsif,
    #[token("else")]
    #[display("else")]
    Else,
    #[token("unless")]
    #[display("unless")]
    Unless,
    #[token("while")]
    #[display("while")]
    While,
    #[token("until")]
    #[display("until")]
    Until,
    #[token("my")]
    #[display("my")]
    My,

    #[token("use")]
    #[display("use")]
    Use,
    #[token("package")]
    #[display("package")]
    Package,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(::[a-zA-Z0-9_]+)*")]
    #[display("_0")]
    PackageName(&'src str),

    #[regex(r"(?x) \( | \)")]
    #[display("{_0}")]
    RoundParen(&'src str),
    #[regex(r"(?x) \[ | \]")]
    #[display("{_0}")]
    SquareBracket(&'src str),
    #[regex(r"(?x) \{ | \}")]
    #[display("{_0}")]
    CurlyBrace(&'src str),

    #[token("+")]
    #[display("+")]
    Plus,
    #[token("-")]
    #[display("-")]
    Minus,
    #[token("*")]
    #[display("*")]
    Star,
    #[token("/")]
    #[display("/")]
    Slash,

    #[token("=")]
    #[display("=")]
    Equal,
    #[token(";")]
    #[display(";")]
    Semicolon,
    #[token("$")]
    #[display("$")]
    Dollar,
    #[token("::")]
    #[display("::")]
    DoubleColon,

    #[regex(r"(\$|@|%)?[a-zA-Z_][a-zA-Z0-9_]*", ident, priority = 3)]
    Ident(Ident<'src>),
    // FIXME: add normal decimal number parsing
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    #[display("_0")]
    Number(i64),
}

#[derive(Debug, Clone, Copy, PartialEq, EnumAsInner, derive_more::Display)]
pub enum Ident<'src> {
    #[display("_0")]
    Bare(&'src str),
    #[display("$_0")]
    Scalar(&'src str),
    #[display("@_0")]
    Array(&'src str),
    #[display("%_0")]
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
