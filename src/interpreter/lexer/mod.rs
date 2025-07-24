mod token;

use logos::Logos;
use token::Token;

pub struct Lexer<'src> {
    pub(crate) source: &'src str,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source
        }
    }

    pub fn tokens(&mut self) -> impl Iterator<Item = Token<'src>> {
        Token::lexer(self.source).filter_map(|maybe_token| maybe_token.ok())
    }
}