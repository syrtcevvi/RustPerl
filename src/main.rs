mod interpreter;

use crate::interpreter::PerlInterpreter;

fn main() {
    let mut interpreter = PerlInterpreter::new();
    interpreter.execute(include_str!("../test.pl"));
}
