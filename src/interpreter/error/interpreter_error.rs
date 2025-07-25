use std::ops::Range;

use crate::interpreter::error::LexingError;

#[derive(Debug)]
pub struct InterpreterError {
    pub(crate) kind: InterpreterErrorKind,
    pub(crate) span: Range<usize>,
}

/// High-level interpreter error type
#[derive(Debug)]
pub enum InterpreterErrorKind {
    Lexing(LexingError),
}
