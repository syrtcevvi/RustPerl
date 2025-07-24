use logos::Logos;

use crate::interpreter::error::LexingError;

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Logos)]
#[logos(error = LexingError)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#[^\n]+")]
pub enum Token<'src> {
    #[token("my")]
    My,
    
    #[token("=")]
    Equal,
    #[token(";")]
    Semicolon,
    #[token("$")]
    Dollar,

    #[regex("[a-zA-Z]+")]
    Ident(&'src str),
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i64)
}