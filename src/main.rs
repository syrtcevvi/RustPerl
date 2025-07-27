mod cli;
mod interpreter;

use crate::{
    cli::{CliArgs, Parser},
    interpreter::PerlInterpreter,
};

use rustyline::DefaultEditor;

fn main() {
    let cli_args = CliArgs::parse();

    let mut interpreter = PerlInterpreter::new();
    if let Some(path_to_script) = cli_args.path_to_script {
        interpreter.execute("todo!");
    } else {
        run_repl(&mut interpreter);
    }
}

fn run_repl(_interpreter: &mut PerlInterpreter) -> rustyline::Result<()> {
    let mut rl = DefaultEditor::new()?;

    let mut line_i = 1;
    loop {
        let read_line = rl.readline(&format!("#{line_i}>>>"));
        match read_line {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                println!("read line: {line}");
            }
            Err(_) => {
                println!("Bye-bye!");
                break;
            }
        }

        line_i += 1;
    }

    Ok(())
}
