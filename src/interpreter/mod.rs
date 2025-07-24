mod lexer;
mod error;

use lexer::Lexer;

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

        dbg!(tokens);
    }
}