use minilisp_parser::{parse_source, Item, Value};
use minilisp_vm::VirtualMachine;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use minilisp_util::color;
fn print_error<T: std::fmt::Display>(error: T) {
    eprintln!(
        "{}",
        [color::ansi("Error:", 196, 16), color::ansi(error.to_string(), 220, 16),].join(" ")
    );
}
fn main() -> Result<()> {
    let mut vm = VirtualMachine::new();
    print!("\x1b[2J\x1b[3J\x1b[H");
    println!("minilisp VM version {}", env!("CARGO_PKG_VERSION"));
    let mut rl = DefaultEditor::new()?;

   if rl.load_history(".minilisp.history").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(": ");
        match readline {
            Ok(line) => {
                match line.as_str() {
                    "@" => {
                        println!("{:#?}", vm.symbols());
                        continue
                    }
                    _ => {}
                }
                match parse_source(&line) {
                    Ok(ast) => {
                        match vm.eval_ast(match ast.len() {
                            0 => Item::Value(Value::Nil),
                            1 => ast.front().map(Clone::clone).unwrap(),
                            _ => Item::List(ast),
                        }) {
                            Ok(value) => {
                                println!("{:#?}", value);
                            },
                            Err(error) => print_error(error),
                        }
                    },
                    Err(error) => print_error(error),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }
    rl.save_history(".minilisp.history")?;
    Ok(())
}
