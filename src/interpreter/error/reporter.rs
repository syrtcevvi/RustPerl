
use ariadne::{Label, Report, ReportKind, Source};

use crate::interpreter::error::{InterpreterError, InterpreterErrorKind, LexingError};

pub struct Reporter;

impl Reporter {
    pub fn new() -> Self {
        Self
    }

    pub fn report(source: &str, error: InterpreterError, file_name: Option<String>) {
        let file_name_or_repl = file_name.unwrap_or("REPL".to_owned());
        match error.kind {
            InterpreterErrorKind::Lexing(lexing_error) => match lexing_error {
                LexingError::UnknownCharacter => {
                    Report::build(ReportKind::Error, (&file_name_or_repl, error.span.clone()))
                        .with_message("Unknown character")
                        .with_label(
                            Label::new((&file_name_or_repl, error.span))
                                .with_message("Unknown character here, remove it"),
                        )
                        .finish()
                        .print((&file_name_or_repl, Source::from(source)))
                        .unwrap();
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
