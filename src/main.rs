mod interpreter;

use crate::interpreter::PerlInterpreter;

fn main() {
    let mut interpreter = PerlInterpreter::new();
    interpreter.execute("my $a = 42; my $b = 123;");
}
