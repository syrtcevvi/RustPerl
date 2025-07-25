mod error;
mod lexer;
mod parser;

use lexer::Lexer;

use crate::interpreter::error::Reporter;

pub struct PerlInterpreter<'src> {
    lexer: Lexer<'src>,
}

impl<'src> PerlInterpreter<'src> {
    pub fn new() -> Self {
        Self {
            lexer: Lexer::new(""),
        }
    }

    pub fn execute(&mut self, source: &'src str) {
        self.lexer.source = source;
        let tokens = self.lexer.tokens().collect::<Vec<_>>();

        dbg!(&tokens);

        for token in tokens {
            if let Err(err) = token {
                Reporter::report(source, err, None);
            }
        }
    }
}
