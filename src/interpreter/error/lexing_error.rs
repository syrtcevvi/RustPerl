use std::num::ParseIntError;

#[derive(Debug, Default, PartialEq, Clone)]
pub enum LexingError {
    InvalidNumber(String),
    #[default]
    Other,
}

impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => Self::InvalidNumber("overflow error".to_owned()),
            _ => Self::InvalidNumber("other error".to_owned()),
        }
    }
}