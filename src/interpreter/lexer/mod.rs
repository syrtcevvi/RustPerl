mod token;


use logos::Logos;

use crate::interpreter::{
    error::{InterpreterError, InterpreterErrorKind},
    lexer::token::TokenKind,
};

use token::Token;

pub struct Lexer<'src> {
    pub(crate) source: &'src str,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source }
    }

    pub fn tokens(&mut self) -> impl Iterator<Item = Result<Token<'src>, InterpreterError>> {
        TokenKind::lexer(self.source)
            .spanned()
            .map(|(token_kind, span)| {
                token_kind
                    .map_err(|err| InterpreterError {
                        kind: InterpreterErrorKind::Lexing(err),
                        span: span.clone(),
                    })
                    .map(|kind| Token { kind, span })
            })
    }
}
