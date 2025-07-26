use ariadne::{Label, Report, ReportKind, Source};

use crate::interpreter::error::{
    InterpreterError, InterpreterErrorKind, LexingError, ParsingError,
};

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
            InterpreterErrorKind::Parsing(parsing_error) => match parsing_error {
                ParsingError::SyntaxError(message) => {
                    todo!()
                }
            },
            InterpreterErrorKind::UnexpectedEndOfInput => {
                Report::build(ReportKind::Error, (&file_name_or_repl, error.span.clone()))
                    .with_message("Parsing error")
                    .with_label(
                        Label::new((&file_name_or_repl, error.span))
                            .with_message("Unexpected end of input"),
                    )
                    .finish()
                    .print((&file_name_or_repl, Source::from(source)))
                    .unwrap();
            }
        }
    }
}
