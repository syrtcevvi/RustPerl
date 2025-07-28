mod token;

use logos::Logos;

use crate::interpreter::error::{InterpreterError, InterpreterErrorKind};

pub use crate::interpreter::lexer::token::{Ident, Token, TokenKind};

pub struct Lexer<'src> {
    pub(crate) source: &'src str,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source }
    }

    pub fn tokens(&self) -> impl Iterator<Item = Result<Token<'src>, InterpreterError>> {
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
