mod error;
mod lexer;
mod parser;

use crate::interpreter::error::Reporter;

use lexer::Lexer;
use parser::Parser;

pub struct PerlInterpreter {}

impl PerlInterpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&mut self, source: &str) {
        let lexer = Lexer::new(source);
        let tokens_without_errors = lexer
            .tokens()
            // Probably it's more appropriate to decide what to do with an error based on the type of the error
            .filter_map(|maybe_token| {
                if let Err(error) = maybe_token {
                    Reporter::report(source, error.clone(), None);
                    return None;
                }
                Some(maybe_token.unwrap())
            });

        let mut parser = Parser::new(tokens_without_errors);
        let ast = parser.parse();

        dbg!(&ast);

        // TODO allow multiple parsing errors
        if let Err(error) = ast {
            Reporter::report(source, error, None);
        }
    }
}
