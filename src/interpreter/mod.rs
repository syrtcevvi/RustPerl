mod error;
mod lexer;
mod parser;

use crate::interpreter::error::Reporter;

use lexer::Lexer;
use parser::Parser;

pub struct PerlInterpreter {
    // lexer: Lexer<'src>,
}

impl PerlInterpreter {
    pub fn new() -> Self {
        Self {
            // lexer: Lexer::new(""),
        }
    }

    pub fn execute(&mut self, source: &str) {
        let mut lexer = Lexer::new(source);
        // TODO process lexer errors?
        let mut parser = Parser::new(lexer.tokens().filter_map(|t| t.ok()));
        let ast = parser.parse();

        dbg!(&ast);

        if let Err(error) = ast {
            Reporter::report(source, error, None);
        }

        // let tokens = self.lexer.tokens().collect::<Vec<_>>();

        // dbg!(&tokens);

        // for token in tokens {
        //     if let Err(err) = token {
        //         Reporter::report(source, err, None);
        //     }
        // }
    }
}
