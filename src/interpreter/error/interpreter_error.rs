use std::ops::Range;

use crate::interpreter::error::{LexingError, ParsingError};

#[derive(Debug, Clone)]
pub struct InterpreterError {
    pub(crate) kind: InterpreterErrorKind,
    pub(crate) span: Range<usize>,
}

/// High-level interpreter error type
#[derive(Debug, Clone)]
pub enum InterpreterErrorKind {
    Lexing(LexingError),
    Parsing(ParsingError),
    UnexpectedEndOfInput,
}

impl InterpreterError {
    pub fn new_lexing(kind: LexingError, span: Range<usize>) -> Self {
        Self {
            kind: InterpreterErrorKind::Lexing(kind),
            span,
        }
    }

    pub fn new_parsing(kind: ParsingError, span: Range<usize>) -> Self {
        Self {
            kind: InterpreterErrorKind::Parsing(kind),
            span,
        }
    }

    pub fn new_unexpected_eoi(last_token_span: Range<usize>) -> Self {
        Self {
            kind: InterpreterErrorKind::UnexpectedEndOfInput,
            span: last_token_span,
        }
    }
}
